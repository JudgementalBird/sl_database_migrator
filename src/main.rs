use std::{sync::Arc, time::Duration};
use colored::{Color, Colorize};
use new_seacreatures::NewSeaCreatures;
use new_withdrawal::NewWithdrawal;
use old_seacreatures::OldSeaCreatures;
use old_withdrawal::OldWithdrawal;
use rusqlite::{Connection, Error};
use small_types::{SteamId, UnixTime};
use tokio::sync::Mutex;

mod new_seacreatures;
mod old_seacreatures;
mod small_types;
mod new_withdrawal;
mod old_withdrawal;
mod handy;

const NEW_AMOUNT_SEA_CREATURE_TYPES: usize = 67; //13..=79


struct SLDatabase {
	pub connection: Arc<Mutex<Connection>>
}
impl SLDatabase {
	pub async fn ensure_withdrawals_table_exists(&self) {
		let locked_fish_database = self.connection.lock().await;
		locked_fish_database.execute(
			"CREATE TABLE IF NOT EXISTS withdrawals (
			crane_id INTEGER NOT NULL,
			steam_id BIGINT NOT NULL,
			specific_withdrawn BLOB NOT NULL,
			received_at BIGINT NOT NULL
			)",()
		).expect("Failed at creating a table if (one doesn't exist)..");
	}
	pub async fn setup(location: &str) -> Result<Self,Error> {
		let database = Self {
                connection: Arc::new(Mutex::new(
                    Connection::open(location)?
                    ))
			};
		
		database.ensure_withdrawals_table_exists().await;

		Ok(database)
	}
}



async fn load_entire_db_into_mem_representation(unlocked_connection: Arc<Mutex<Connection>>) -> Result<Vec<OldWithdrawal>,Error> {
    let connection = unlocked_connection.lock().await;
    let mut statement = connection.prepare("SELECT * FROM withdrawals")?;
    let res = statement.query_map([], |row| {
        Ok((
            row.get(0)?,
            row.get(1)?,
            row.get(2)?,
            row.get(3)?,
        ))
    })?;
    let mem_representation = res.map(|row| {
        let row = row.unwrap();
        let specific_withdrawn: Vec<u8> = row.2;
        OldWithdrawal {
            crane_id: row.0,
            steam_id: SteamId::from_u64(row.1),
            specific_withdrawn: bincode::deserialize(&specific_withdrawn).unwrap(),
            received_at: UnixTime::from_u64(row.3),
        }
    })
    .collect::< Vec<OldWithdrawal> >();
    Ok(mem_representation)
}
async fn write_entire_mem_representation_into_provided_db(unlocked_connection: Arc<Mutex<Connection>>, mem_representation: Vec<NewWithdrawal>) -> Result<(),Error> {
    let mut connection = unlocked_connection.lock().await;
    let transaction = connection.transaction()?;

    {
        let mut statement = transaction.prepare("INSERT INTO withdrawals (crane_id, steam_id, specific_withdrawn, received_at) VALUES (?1, ?2, ?3, ?4)")?;
        for new_withdrawal in mem_representation {
            let new_sea_creatures = &new_withdrawal.specific_withdrawn.creatures.to_vec();
            statement.execute(
                (new_withdrawal.crane_id, new_withdrawal.steam_id.to_u64(), bincode::serialize(new_sea_creatures).unwrap(), new_withdrawal.received_at.clone().to_u64())
            ).unwrap();
        }
    }
    
    transaction.commit()?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    println!("{}","Database migrator has been opened !".magenta());
    let db = SLDatabase::setup("./fish_addon_db.db3").await.expect("Failed to open DB!");
    println!("DB opened successfully");
    let new_mem_representation = {
        let mem_representation = load_entire_db_into_mem_representation(db.connection.clone()).await?;
        println!("{}","Loaded ENTIRE DB into memory successfully!".green());

        let mut new_withdrawals: Vec<NewWithdrawal> = Vec::new();

        for (index, old_withdrawal) in mem_representation.iter().enumerate() {
            println!("Withdrawal {index}!");
            let mut new_specific_withdrawn = NewSeaCreatures {creatures: [0;NEW_AMOUNT_SEA_CREATURE_TYPES]};
            for (species,quantity) in old_withdrawal.specific_withdrawn.creatures.iter().enumerate() {
                if species > 66  {
                    if *quantity > 0_u32 {
                        println!("{}",format!("DISCARDING QUANTITY: {quantity} OF SPECIES/INDEX: {species}").bold().red());
                    } else {
                        println!("{}",format!("DISCARDING QUANTITY: {quantity} OF SPECIES/INDEX: {species}").bold().cyan());
                    }
                } else {
                    new_specific_withdrawn.creatures[species] = quantity.clone()
                }
            }
            let res = NewWithdrawal {
                crane_id: old_withdrawal.crane_id,
                steam_id: old_withdrawal.steam_id,
                specific_withdrawn: new_specific_withdrawn,
                received_at: old_withdrawal.received_at.clone(),
            };
            new_withdrawals.push(res);
            
            println!(
                "{} {} {}",
                "-".repeat(40).magenta(),
                "Success".magenta(),
                "-".repeat(100).magenta());
            
            let (data_string, index_string) = new_withdrawals[index].specific_withdrawn.creatures.iter()
                .enumerate()
                .map(|(species, &quantity)| {
                    let to_use = match quantity {
                        0 => Color::Yellow,
                        _ => Color::Green
                    };
                    let data = if species >= 10 && quantity < 10 {
                        format!("{}, ",
                            format!("0{}", quantity).color(to_use)
                        )
                    } else {
                        format!("{}, ",quantity.to_string().color(to_use))
                    };

                    let index = format!("{}, ",species.to_string().yellow());

                    (data,index)
                })
                .fold((String::new(), String::new()), |(mut data_acc, mut index_acc), (data, index)| {
                    data_acc.push_str(&data);
                    index_acc.push_str(&index);
                    (data_acc, index_acc)
                });
        
            println!(" Indexes: {}", index_string);
            println!("Vec<u32>: {}", data_string);
            println!("    Unix: {:?}", new_withdrawals[index].received_at);

            //tokio::time::sleep(Duration::from_millis(100)).await;
        };
        new_withdrawals
    };
    
    println!("\n--Inserting into new database--\n");

    let new_db = SLDatabase::setup("./fish_addon_new_db.db3")
        .await
        .expect("Failed to open new DB!");

    write_entire_mem_representation_into_provided_db(new_db.connection,new_mem_representation).await?;

    println!("{}","\n--Wrote entire memory representation into new database!--\n".bold().green());

    Ok(())
}