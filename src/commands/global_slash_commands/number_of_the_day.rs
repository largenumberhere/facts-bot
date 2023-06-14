use crate::bot;
use crate::command_result::{CommandError, CommandSuccess, ToCommandResult};
use crate::global_slash_command::{GetSlashCommandDetails, GlobalSlashCommandDetails};
use futures::FutureExt;
use serde::{Deserialize, Serialize};
use serenity::client::Context;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::application::interaction::Interaction;

pub struct NumberOfTheDay {}

impl GetSlashCommandDetails for NumberOfTheDay {
    fn get_slash_command_details() -> GlobalSlashCommandDetails {
        GlobalSlashCommandDetails {
            name: "number_of_the_day".to_string(),
            options: vec![],
            description:
                "get information about the number of the day according to https://api.math.tools/"
                    .to_string(),
            handler: |command_interaction, context, interaction| {
                handler(command_interaction, context, interaction).boxed()
            },
            force_command_update: None,
        }
    }
}

async fn handler(
    _command_interaction: &ApplicationCommandInteraction,
    _context: &Context,
    _interaction: &Interaction,
) -> Result<CommandSuccess, CommandError> {
    let uri = "https://api.math.tools/numbers/nod"
        .parse()
        .to_command_result()?;
    let json = bot::HttpClient::https_get_json(uri)
        .await
        .to_command_result()?;
    let number_of_the_day_response: Root =
        serde_json::from_str(json.as_str()).to_command_result()?;
    let number_of_the_day = number_of_the_day_response.contents.nod.numbers.number;

    let reply = format!("The number of the day is: {}", number_of_the_day);

    Ok(CommandSuccess::SuccessWithReply(reply))
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub success: Success,
    pub copyright: Copyright,
    pub contents: Contents,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Success {
    pub total: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Copyright {
    pub copyright: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Contents {
    pub nod: Nod,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Nod {
    pub category: Category,
    pub numbers: Numbers,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Category {
    pub name: String,
    pub description: String,
    pub background: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Numbers {
    pub number: String,
    pub uuid: String,
    pub id: String,
    pub names: Names,
    pub bases: Bases,
    pub numerals: Numerals,
    #[serde(rename = "general-facts")]
    pub general_facts: GeneralFacts,
    #[serde(rename = "prime-facts")]
    pub prime_facts: PrimeFacts,
    pub recreational: Recreational,
    pub category: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Names {
    pub nominal: Nominal,
    pub cardinal: Cardinal,
    pub ordinal: Ordinal,
    #[serde(rename = "us_currency")]
    pub us_currency: UsCurrency,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Nominal {
    pub name: String,
    pub description: String,
    pub value: String,
    pub display: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cardinal {
    pub name: String,
    pub description: String,
    pub value: String,
    pub display: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ordinal {
    pub name: String,
    pub description: String,
    pub value: String,
    pub display: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UsCurrency {
    pub name: String,
    pub description: String,
    pub value: String,
    pub display: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bases {
    pub binary: Binary,
    pub ternary: Ternary,
    pub quaternary: Quaternary,
    pub quinary: Quinary,
    pub senary: Senary,
    pub octal: Octal,
    pub duodecimal: Duodecimal,
    pub hexadecimal: Hexadecimal,
    pub vigesimal: Vigesimal,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Binary {
    pub name: String,
    pub description: String,
    pub value: String,
    pub display: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ternary {
    pub name: String,
    pub description: String,
    pub value: String,
    pub display: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Quaternary {
    pub name: String,
    pub description: String,
    pub value: String,
    pub display: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Quinary {
    pub name: String,
    pub description: String,
    pub value: String,
    pub display: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Senary {
    pub name: String,
    pub description: String,
    pub value: String,
    pub display: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Octal {
    pub name: String,
    pub description: String,
    pub value: String,
    pub display: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Duodecimal {
    pub name: String,
    pub description: String,
    pub value: String,
    pub display: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Hexadecimal {
    pub name: String,
    pub description: String,
    pub value: String,
    pub display: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Vigesimal {
    pub name: String,
    pub description: String,
    pub value: String,
    pub display: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Numerals {
    pub roman: Roman,
    pub chinese: Chinese,
    pub egyptian: Egyptian,
    pub babylonian: Babylonian,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Roman {
    pub name: String,
    pub description: String,
    pub value: String,
    pub display: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Chinese {
    pub name: String,
    pub description: String,
    pub value: String,
    pub display: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Egyptian {
    pub name: String,
    pub description: String,
    pub value: String,
    pub display: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Babylonian {
    pub name: String,
    pub description: String,
    pub value: String,
    pub display: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeneralFacts {
    pub odd: Odd,
    pub even: Even,
    pub palindrome: Palindrome,
    pub triangle: Triangle,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Odd {
    pub name: String,
    pub description: String,
    pub value: bool,
    pub display: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Even {
    pub name: String,
    pub description: String,
    pub value: bool,
    pub display: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Palindrome {
    pub name: String,
    pub description: String,
    pub value: bool,
    pub display: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Triangle {
    pub name: String,
    pub description: String,
    pub value: bool,
    pub display: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrimeFacts {
    pub prime: Prime,
    pub perfect: Perfect,
    pub mersenne: Mersenne,
    pub fermat: Fermat,
    pub fibonacci: Fibonacci,
    pub partition: Partition,
    pub pell: Pell,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Prime {
    pub name: String,
    pub description: String,
    pub value: bool,
    pub display: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Perfect {
    pub name: String,
    pub description: String,
    pub value: bool,
    pub display: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mersenne {
    pub name: String,
    pub description: String,
    pub value: bool,
    pub display: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fermat {
    pub name: String,
    pub description: String,
    pub value: bool,
    pub display: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fibonacci {
    pub name: String,
    pub description: String,
    pub value: bool,
    pub display: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Partition {
    pub name: String,
    pub description: String,
    pub value: bool,
    pub display: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pell {
    pub name: String,
    pub description: String,
    pub value: bool,
    pub display: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Recreational {
    pub reverse: Reverse,
    pub digitssum: Digitssum,
    pub noofdigits: Noofdigits,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Reverse {
    pub name: String,
    pub description: String,
    pub value: String,
    pub display: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Digitssum {
    pub name: String,
    pub description: String,
    pub value: i64,
    pub display: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Noofdigits {
    pub name: String,
    pub description: String,
    pub value: i64,
    pub display: i64,
}
/***
Json sample:

    {
      "success": {
        "total": 1
      },
      "copyright": {
        "copyright": "2019-21 https://math.tools"
      },
      "contents": {
        "nod": {
          "category": {
            "name": "nod",
            "description": "Number of the day.",
            "background": ""
          },
          "numbers": {
            "number": "75282",
            "uuid": "75282",
            "id": "75282",
            "names": {
              "nominal": {
                "name": "nominal",
                "description": "Nominal",
                "value": "75282",
                "display": "75282"
              },
              "cardinal": {
                "name": "cardinal",
                "description": "Cardinal",
                "value": "seventy-five thousand two hundred eighty-two",
                "display": "seventy-five thousand two hundred eighty-two"
              },
              "ordinal": {
                "name": "ordinal",
                "description": "Ordinal",
                "value": "75,282nd",
                "display": "75,282nd"
              },
              "us_currency": {
                "name": "us_currency",
                "description": "This number as US currency",
                "value": "seventy-five thousand two hundred eighty-two dollars",
                "display": "seventy-five thousand two hundred eighty-two dollars"
              }
            },
            "bases": {
              "binary": {
                "name": "binary",
                "description": "<a target='_blank' href='http://funtranslations.com/binary' title='Binary Translator'>Base 2 (Binary)</a>",
                "value": "10010011000010010",
                "display": "10010011000010010<sub>2</sub>"
              },
              "ternary": {
                "name": "ternary",
                "description": "Base 3 (Ternary)",
                "value": "10211021020",
                "display": "10211021020<sub>3</sub>"
              },
              "quaternary": {
                "name": "quaternary",
                "description": "Base 4 (Quaternary)",
                "value": "102120102",
                "display": "102120102<sub>4</sub>"
              },
              "quinary": {
                "name": "quinary",
                "description": "Base 5 (Quinary)",
                "value": "4402112",
                "display": "4402112<sub>5</sub>"
              },
              "senary": {
                "name": "senary",
                "description": "Base 6 (Senary)",
                "value": "1340310",
                "display": "1340310<sub>6</sub>"
              },
              "octal": {
                "name": "octal",
                "description": "Base 8 (Octal)",
                "value": "223022",
                "display": "223022<sub>8</sub>"
              },
              "duodecimal": {
                "name": "duodecimal",
                "description": "Base 12 (Duodecimal)",
                "value": "37696",
                "display": "37696<sub>12</sub>"
              },
              "hexadecimal": {
                "name": "vexadecimal",
                "description": "Base 16 (Hexadecimal)",
                "value": "12612",
                "display": "12612<sub>16</sub>"
              },
              "vigesimal": {
                "name": "vigesimal",
                "description": "Base 20 (Vigesimal)",
                "value": "9842",
                "display": "9842<sub>20</sub>"
              }
            },
            "numerals": {
              "roman": {
                "name": "roman",
                "description": "75282 in <a href=\"/numbers/to-roman/\">Roman Numeral</a>",
                "value": "<span class='u'>L</span><span class='u'>X</span><span class='u'>X</span><span class='u'>V</span>CCLXXXII",
                "display": "<span class='u'>L</span><span class='u'>X</span><span class='u'>X</span><span class='u'>V</span>CCLXXXII"
              },
              "chinese": {
                "name": "chinese",
                "description": "75282 in <a href=\"/numbers/to-chinese/\">Chinese Numeral</a>",
                "value": "&#26578;&#33836;&#20237;&#20191;&#36019;&#20336;&#25420;&#25342;&#36019;",
                "display": "&#26578;&#33836;&#20237;&#20191;&#36019;&#20336;&#25420;&#25342;&#36019;"
              },
              "egyptian": {
                "name": "egyptian",
                "description": "75282 in <a href=\"/numbers/to-egyptian/\">Egyptian Numeral</a>",
                "value": "&#x130b3;&#x131c0;&#x13363;&#x1338d;&#x133fb;",
                "display": "&#x130b3;&#x131c0;&#x13363;&#x1338d;&#x133fb;"
              },
              "babylonian": {
                "name": "babylonian",
                "description": "75282 in <a href=\"/numbers/to-babylonian/\">Babylonian Numeral</a>",
                "value": "<img src=\"https://math.tools/img/babnumbers/bab_20.gif\" alt=\"20\"> &nbsp; <img src=\"https://math.tools/img/babnumbers/bab_54.gif\" alt=\"54\"> &nbsp; <img src=\"https://math.tools/img/babnumbers/bab_42.gif\" alt=\"42\"> &nbsp; ",
                "display": "<img src=\"https://math.tools/img/babnumbers/bab_20.gif\" alt=\"20\"> &nbsp; <img src=\"https://math.tools/img/babnumbers/bab_54.gif\" alt=\"54\"> &nbsp; <img src=\"https://math.tools/img/babnumbers/bab_42.gif\" alt=\"42\"> &nbsp; "
              }
            },
            "general-facts": {
              "odd": {
                "name": "odd",
                "description": "Is 75282 an odd number?",
                "value": false,
                "display": "75282 is NOT an odd number"
              },
              "even": {
                "name": "even",
                "description": "Is 75282 an even number?",
                "value": true,
                "display": "75282 is  an even number"
              },
              "palindrome": {
                "name": "palindrome",
                "description": "Is 75282 a palindrome?",
                "value": false,
                "display": "75282 is  NOT a palindrome number"
              },
              "triangle": {
                "name": "triangle",
                "description": "Is 75282 a triangle number?",
                "value": false,
                "display": "75282 is  NOT a triangle number"
              }
            },
            "prime-facts": {
              "prime": {
                "name": "prime",
                "description": "Is 75282 a Prime Number?",
                "value": false,
                "display": "75282 is NOT a  prime"
              },
              "perfect": {
                "name": "perfect",
                "description": "Is 75282 a perfect number?",
                "value": false,
                "display": "75282 is NOT a perfect number"
              },
              "mersenne": {
                "name": "mersenne",
                "description": "Is 75282 a Mersenne Prime?",
                "value": false,
                "display": "75282 is NOT a Mersenne prime"
              },
              "fermat": {
                "name": "fermat",
                "description": "Is 75282 a Fermat Prime?",
                "value": false,
                "display": "75282 is NOT a Fermat prime"
              },
              "fibonacci": {
                "name": "fibonacci",
                "description": "Is 75282 a Fibonacci Prime?",
                "value": false,
                "display": "75282 is NOT a Fibonacci prime"
              },
              "partition": {
                "name": "partition",
                "description": "Is 75282 a Partition Prime?",
                "value": false,
                "display": "75282 is NOT a Partition prime"
              },
              "pell": {
                "name": "pell",
                "description": "Is 75282 a Pell Prime?",
                "value": false,
                "display": "75282 is NOT a Pell prime"
              }
            },
            "recreational": {
              "reverse": {
                "name": "reverse",
                "description": "Number 75282 reversed",
                "value": "28257",
                "display": "28257"
              },
              "digitssum": {
                "name": "digitssum",
                "description": "Sum of the digits",
                "value": 24,
                "display": 24
              },
              "noofdigits": {
                "name": "noofdigits",
                "description": "No of digits",
                "value": 5,
                "display": 5
              }
            },
            "category": "nod"
          }
        }
      }
    }
/
 */
