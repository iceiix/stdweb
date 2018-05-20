#[macro_use]
extern crate stdweb;

#[macro_use]
extern crate serde_derive;

extern crate serde;

use std::collections::HashMap;
use std::cell::RefCell;

use stdweb::Value;

use stdweb::traits::*;
use stdweb::web::{
    HtmlElement,
    Element,
    window,
    document,
    IEventTarget,
    EventTarget
};

use stdweb::web::event::{ConcreteEvent, IEvent};

use stdweb::web::event::{
    IDBSuccessEvent,
    IDBVersionChangeEvent,
    IDBCompleteEvent,
    IDBErrorEvent
};

use stdweb::web::indexeddb::{
    IDBOpenDBRequest,
    IDBDatabase,
    IDBRequest,
    DBRequest,
    IDBObjectStore,
    IDBObjectStoreIndexSharedMethods
};

use stdweb::unstable::TryInto;

#[derive(Serialize, Deserialize)]
struct Note {
  title: String,
  body: String
}

js_serializable!( Note );
js_deserializable!( Note );

fn main() {
    stdweb::initialize();

    let title_input = document().query_selector("#title").unwrap().unwrap();
    let body_input = document().query_selector("#body").unwrap().unwrap();
    let form = document().query_selector("form").unwrap().unwrap();
    let submit_btn = document().query_selector("form button");

    let db: RefCell<Option<IDBDatabase>> = RefCell::new(None);
    
    // Open our database; it is created if it doesn't already exist
    // (see onupgradeneeded below)
    let request = window().indexed_db().open("notes", 1);

    // onerror handler signifies that the database didn't open successfully
    request.add_event_listener( |event: IDBErrorEvent| {
        js!(
            console.log("Database failed to open");
        );
    });

    // onsuccess handler signifies that the database opened successfully
    request.add_event_listener( |event: IDBSuccessEvent| {
        js!(
            console.log("Database opened succesfully");
            );

        let db_request: IDBOpenDBRequest = event.target().unwrap().try_into().unwrap();
        // Store the opened database object in the db variable. This is used a lot below
        let db_ : IDBDatabase = db_request.result().try_into().unwrap();

        //db.replace(Some(db_));
        // Run the displayData() function to display the notes already in the IDB
        display_data(db_);
    });
    
    request.add_event_listener( |event: IDBVersionChangeEvent| {
    	let db_request: IDBOpenDBRequest = event.target().unwrap().try_into().unwrap();
        let db: IDBDatabase = db_request.result().try_into().unwrap();

        // Create an objectStore to store our notes in (basically like a single table)
        // including a auto-incrementing key
        let mut store_options = HashMap::new();
        store_options.insert("keyPath", "id");
        store_options.insert("autoIncrement", "true");
        let object_store = db.create_object_store("notes", Value::from(store_options));
        
        // Define what data items the objectStore will contain
        let mut title_options = HashMap::new();
        title_options.insert("unique", false);
        object_store.create_index("title", "title", Value::from(title_options));
       
         let mut body_options = HashMap::new();
        body_options.insert("unique", false);
        object_store.create_index("body", "body", Value::from(body_options));

        js!(
            console.log("Database setup complete");
        );

    });

    // Define the displayData() function
    fn display_data(db: IDBDatabase) {
        let list = document().query_selector("ul").unwrap().unwrap();
        
        // Here we empty the contents of the list element each time the display is updated
        // If you ddn't do this, you'd get duplicates listed each time a new note is added
        while list.first_child().is_some() {
            list.remove_child(&list.first_child().unwrap());
        }

        // Open our object store and then get a cursor - which iterates through all the
        // different data items in the store
        let object_store = db.transaction("notes", "readonly").object_store("notes");
        
        object_store.open_cursor(None, None)
            .add_event_listener( |e: IDBSuccessEvent| {
            
                // Get a reference to the cursor
            
                let cursor = e.target();
                    
                    //.result;
/*            
      // If there is still another data item to iterate through, keep running this code
            if(cursor) {
                // Create a list item, h3, and p to put each data item inside when displaying it
                // structure the HTML fragment, and append it inside the list
                let listItem = document.createElement('li');
                let h3 = document.createElement('h3');
                let para = document.createElement('p');
                
                listItem.appendChild(h3);
                listItem.appendChild(para);
                list.appendChild(listItem);

                // Put the data from the cursor inside the h3 and para
                h3.textContent = cursor.value.title;
                para.textContent = cursor.value.body;
                
                // Store the ID of the data item inside an attribute on the listItem, so we know
                // which item it corresponds to. This will be useful later when we want to delete items
                listItem.setAttribute('data-note-id', cursor.value.id);
                
        // Create a button and place it inside each listItem
                let deleteBtn = document.createElement('button');
                listItem.appendChild(deleteBtn);
                deleteBtn.textContent = 'Delete';
                
                // Set an event handler so that when the button is clicked, the deleteItem()
                // function is run
                deleteBtn.onclick = deleteItem;

                // Iterate to the next item in the cursor
                cursor.continue();
            } else {
                // Again, if list item is empty, display a 'No notes stored' message
                if(!list.firstChild) {
                    let listItem = document.createElement('li');
                    listItem.textContent = 'No notes stored.'
          list.appendChild(listItem);
                }
                // if there are no more cursor items to iterate through, say so
                console.log('Notes all displayed');
            }
*/
        });
        
         
    }
}
