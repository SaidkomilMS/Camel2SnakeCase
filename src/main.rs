use std::collections::HashMap;

use serde_json::{Result, Value};
use serde_derive::{Deserialize, Serialize};


#[derive(Debug, Deserialize, Serialize)]
struct ResponseItemCamel {
    key: String,
    labelRu: String,
    labelUz: String,
    value: Option<String>
}

#[derive(Debug, Deserialize, Serialize)]
struct ResponseItemSnake {
    key: String,
    label_ru: String,
    label_uz: String,
    value: Option<String>
}

impl ResponseItemSnake {
    pub fn from_camel(camel: ResponseItemCamel) -> Self {
        Self {
            key: camel.key,
            label_ru: camel.labelRu,
            label_uz: camel.labelUz,
            value: camel.value
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct TransactionCamel {
    transactionId: String,
    statusText: String,
    status: String,
    time: u64,
    response: Vec<ResponseItemCamel>
}

#[derive(Debug, Deserialize, Serialize)]
struct TransactionSnake {
    transaction_id: String,
    status_text: String,
    status: String,
    time: u64,
    response: Vec<ResponseItemSnake>
}

impl TransactionSnake {
    pub fn from_camel(camel: TransactionCamel) -> Self {
        Self {
            transaction_id: camel.transactionId,
            status_text: camel.statusText,
            status: camel.status,
            time: camel.time,
            response: {
                let mut result = Vec::<ResponseItemSnake>::new();
                for i in camel.response {
                    result.push(ResponseItemSnake::from_camel(i));
                };
                result
            }
        }
    }
}

fn main()  -> Result<()> {
    // let input_path = std::env::args().nth(1).unwrap();
    let output_path = std::env::args().nth(1).unwrap();
    
    // let output_path = "new.json";

    let data = r#"
    {
        "transactionId": "123123",
        "statusText": "Success",
        "status": "0",
        "time": 1674674676000,
        "response": [
            {
                "key": "dasdsa",
                "labelRu": "label in russian",
                "labelUz": "label in uzbek",
                "value": "Some value"
            },
            {
                "key": "dasdsa",
                "labelRu": "label in russian",
                "labelUz": "label in uzbek",
                "value": "Some value2"
            },
            {
                "key": "dasdsa",
                "labelRu": "label in russian",
                "labelUz": "label in uzbek",
                "value": null
            }
        ]
    }
    "#;
    // let transaction = {
    //     serde_json::from_str::<TransactionCamel>(data).unwrap()
    // };
    let mut transaction = {
        let missy_secrets = std::fs::read_to_string(&output_path).expect("Error");

        // Load the MissyFoodSchedule structure from the string.
        serde_json::from_str::<TransactionCamel>(&missy_secrets).unwrap()
    };
    let transaction_snake = TransactionSnake::from_camel(transaction);

    std::fs::write(
        output_path,
        serde_json::to_string_pretty(&transaction_snake).unwrap(),
    ).expect("Error");

    Ok(())

}
