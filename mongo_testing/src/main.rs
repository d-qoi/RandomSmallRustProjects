use futures::{StreamExt, TryStreamExt};
use mongodb::{
    bson::{doc, Bson},
    options::{ClientOptions, UpdateOptions},
    Client, Cursor,
};
use serde_json::{json, Value};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client_opts = ClientOptions::parse("mongodb://localhost:27017").await?;
    let client = Client::with_options(client_opts)?;
    let db = client.database("test");
    let collection = db.collection::<Value>("test");
    let ticket_response = json!(
            {
       "total":2,
       "offset":0,
       "limit":10,
       "tickets":[
          {
             "ticketNumber":"FNVL-NIJT-WP021",
             "orderNumber":"FNVL-NIJT-WP",
             "ticketDefinitionId":"d86ffbbd-aa9b-488d-a14b-11752b78e272",
             "name":"Early Bird",
             "price":{
                "amount":"12.34",
                "currency":"USD"
             },
             "free":false,
             "policy":"This is your event ticket. Ticket holders must present their tickets on entry.",
             "orderStatus":"PAID",
             "orderArchived":false,
             "orderFullName":"John Doe",
             "guestDetails":{
                "guestAssigned":false,
                "firstName":"John",
                "lastName":"Doe",
                "email":"john.doe@example.com",
                "contactId":"3ef81e9f-106e-408d-a3ee-3df60838acc6"
             },
             "archived":false,
             "anonymized":false,
             "checkInUrl":"https://www.wixevents.com/check-in/FNVL-NIJT-WP021,ad18d12e-a6a9-4c17-abfa-6ff119479be2",
             "ticketPdfUrl":"https://apps.wix.com/events/doc/tickets/Ticket.pdf?request=<TOKEN>"
          },
          {
             "ticketNumber":"FNVL-O7MZ-0Q021",
             "orderNumber":"FNVL-O7MZ-0Q",
             "ticketDefinitionId":"d86ffbbd-aa9b-488d-a14b-11752b78e272",
             "name":"Early Bird",
             "price":{
                "amount":"11.11",
                "currency":"USD"
             },
             "free":false,
             "policy":"This is your event ticket. Ticket holders must present their tickets on entry.",
             "orderStatus":"OFFLINE_PENDING",
             "orderArchived":false,
             "orderFullName":"Jane Doe",
             "guestDetails":{
                "guestAssigned":false,
                "firstName":"Jane",
                "lastName":"Doe",
                "email":"jane.doe@example.com",
                "contactId":"2b7494bc-550a-47d3-8bba-d22564ae8bdc"
             },
             "archived":false,
             "anonymized":false,
             "checkInUrl":"https://www.wixevents.com/check-in/FNVL-O7MZ-0Q021,ad18d12e-a6a9-4c17-abfa-6ff119479be2",
             "ticketPdfUrl":"https://apps.wix.com/events/doc/tickets/Ticket.pdf?request=<TOKEN>"
          }
       ],
       "facets":{

       }
    });
    let tickets = match ticket_response["tickets"].clone() {
        Value::Array(ticket_list) => ticket_list,
        _ => Vec::new(),
    };
    println!("{:?}", tickets);
    for ticket in tickets {
        println!("{:?}", ticket["ticketNumber"].as_str().unwrap());
        let bson_ticket: Bson = Bson::try_from(ticket.clone()).unwrap();
        let resp = collection
            .update_one(
                doc! {"ticketNumber": ticket["ticketNumber"].as_str().unwrap()},
                doc! {"$set": bson_ticket},
                UpdateOptions::builder().upsert(true).build(),
            )
            .await;
        println!("{:?}", resp);
    }
    let mut cursor: Cursor<Value> = collection.find(doc! {"ticketNumber": "asfd"}, None).await?;
    while let Some(doc) = cursor.next().await {
        println!("{:?}", doc?);
    }
    println!("Hello, world!");
    Ok(())
}
