use entity::projects;
use sea_orm::EntityTrait;
use sea_orm_migration::prelude::*;
use serde_json::json;

use crate::helpers::convert_projects_into_active_model_v2;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        let db_connection = manager.get_connection();

        let make_add: [projects::ActiveModel; 100] = [
            convert_projects_into_active_model_v2(&json!({
          "id": "1097",
          "attributes": {
            "name": "Concreting of Road at Prk 3 to Prk 4, Barangay Bogo\n",
            "code": "1SAIL23 - 8000 - A9",
            "location": "[\"22\"]",
            "cost": 963619.7,
            "contractor": {
              "data": {
                "id": "19",
                "attributes": {
                  "name": "Jenrich Construction",
                  "street_purok": null
                }
              }
            },
            "duration": 90,
            "adjusted": null,
            "total": null,
            "weight": null,
            "prop_sdg": "[]",
            "prop_fund": "19",
            "prop_type": "1",
            "prop_category": "2",
            "prop_sector": "[]",
            "prop_assign": "2",
            "accom_total": 100,
            "accom_value": null,
            "accom_date": null,
            "accom_elapse": null,
            "accom_slippage": null,
            "accom_period": null,
            "accom_todate": null,
            "accom_qty": null,
            "accom_amt": null,
            "date": null,
            "status": "CREATED",
            "start": "2023-06-26",
            "target": "2023-09-24",
            "office": "Engineer's Office",
            "assigned": null,
            "appropriation": 1000000,
            "abc": 0,
            "status_id": {
              "data": null
            },
            "remarks": "100% physically completed based on approved technical plans & specifications\n",
            "entry_type": "IMPLEMENTED",
            "bid_date": null,
            "year": 2023,
            "abc_published": null,
            "prop_takers": null,
            "prop_infra": null,
            "time_extension": null
          }
        }), db_connection).await,
convert_projects_into_active_model_v2(&json!({
          "id": "1098",
          "attributes": {
            "name": "Concreting of Road at Prk Makugihon to Prk Malambuon, Barangay Bulatok",
            "code": "1SAIL23-8000-A10",
            "location": "[\"24\"]",
            "cost": 722995.7,
            "contractor": {
              "data": {
                "id": "1",
                "attributes": {
                  "name": "CTG Construction",
                  "street_purok": "Purok 1"
                }
              }
            },
            "duration": 45,
            "adjusted": null,
            "total": null,
            "weight": null,
            "prop_sdg": "[]",
            "prop_fund": "19",
            "prop_type": "1",
            "prop_category": "2",
            "prop_sector": "[]",
            "prop_assign": "2",
            "accom_total": 100,
            "accom_value": null,
            "accom_date": null,
            "accom_elapse": null,
            "accom_slippage": null,
            "accom_period": null,
            "accom_todate": null,
            "accom_qty": null,
            "accom_amt": null,
            "date": null,
            "status": "CREATED",
            "start": "2023-07-27",
            "target": "2023-09-10",
            "office": "Engineer's Office",
            "assigned": null,
            "appropriation": 800000,
            "abc": 0,
            "status_id": {
              "data": null
            },
            "remarks": "100% physically completed based on approved technical plans & specifications\n\n* Actual completion date: 9/6/2023\n",
            "entry_type": "IMPLEMENTED",
            "bid_date": null,
            "year": 2023,
            "abc_published": null,
            "prop_takers": "KAG.VEDRA/ANGGO",
            "prop_infra": null,
            "time_extension": null
          }
        }), db_connection).await,
convert_projects_into_active_model_v2(&json!({
          "id": "1099",
          "attributes": {
            "name": "Concreting of Road from Prk Malipayon (Zone 2) to Prk Madasigon (Zone 1), Barangay Bulatok\n",
            "code": "1SAIL23-8000-A11",
            "location": "[\"24\"]",
            "cost": 967946.31,
            "contractor": {
              "data": {
                "id": "1",
                "attributes": {
                  "name": "CTG Construction",
                  "street_purok": "Purok 1"
                }
              }
            },
            "duration": null,
            "adjusted": null,
            "total": null,
            "weight": null,
            "prop_sdg": "[]",
            "prop_fund": "19",
            "prop_type": "1",
            "prop_category": "2",
            "prop_sector": "[]",
            "prop_assign": "2",
            "accom_total": 0,
            "accom_value": null,
            "accom_date": null,
            "accom_elapse": null,
            "accom_slippage": null,
            "accom_period": null,
            "accom_todate": null,
            "accom_qty": null,
            "accom_amt": null,
            "date": null,
            "status": "CREATED",
            "start": null,
            "target": null,
            "office": "Engineer's Office",
            "assigned": null,
            "appropriation": 1000000,
            "abc": 0,
            "status_id": {
              "data": null
            },
            "remarks": "Bided but not yet started  as of March 8, 2024",
            "entry_type": "IMPLEMENTED",
            "bid_date": null,
            "year": 2023,
            "abc_published": null,
            "prop_takers": "KAG. VEDRA",
            "prop_infra": null,
            "time_extension": null
          }
        }), db_connection).await,
        convert_projects_into_active_model_v2(&json!({
          "id": "1100",
          "attributes": {
            "name": "Gravelling of Road at Prk Malipayon (Zone 2)Beside Urban Poor Relocation, Barangay Bulatok\n",
            "code": "1SAIL23 - 8000 - A12",
            "location": "[]",
            "cost": 0,
            "contractor": {
              "data": null
            },
            "duration": null,
            "adjusted": null,
            "total": null,
            "weight": null,
            "prop_sdg": "[]",
            "prop_fund": "19",
            "prop_type": "1",
            "prop_category": "4",
            "prop_sector": "[]",
            "prop_assign": "2",
            "accom_total": 0,
            "accom_value": null,
            "accom_date": null,
            "accom_elapse": null,
            "accom_slippage": null,
            "accom_period": null,
            "accom_todate": null,
            "accom_qty": null,
            "accom_amt": null,
            "date": null,
            "status": "CREATED",
            "start": null,
            "target": null,
            "office": "Engineer's Office",
            "assigned": null,
            "appropriation": 332953,
            "abc": 0,
            "status_id": {
              "data": null
            },
            "remarks": null,
            "entry_type": "IMPLEMENTED",
            "bid_date": null,
            "year": 2023,
            "abc_published": null,
            "prop_takers": null,
            "prop_infra": null,
            "time_extension": null
          }
        }), db_connection).await,
convert_projects_into_active_model_v2(&json!({
          "id": "1101",
          "attributes": {
            "name": "Continuation of Concreting of Road at Prk Malipayon (Zone 2) To Connect to North Diversion Road, Barangay Bulatok\n",
            "code": "1SAIL23 - 8000 - A13",
            "location": "[\"24\"]",
            "cost": 385994.5,
            "contractor": {
              "data": {
                "id": "1",
                "attributes": {
                  "name": "CTG Construction",
                  "street_purok": "Purok 1"
                }
              }
            },
            "duration": 45,
            "adjusted": null,
            "total": null,
            "weight": null,
            "prop_sdg": "[]",
            "prop_fund": "19",
            "prop_type": "1",
            "prop_category": "2",
            "prop_sector": "[]",
            "prop_assign": "2",
            "accom_total": 100,
            "accom_value": null,
            "accom_date": null,
            "accom_elapse": null,
            "accom_slippage": null,
            "accom_period": null,
            "accom_todate": null,
            "accom_qty": null,
            "accom_amt": null,
            "date": null,
            "status": "CREATED",
            "start": "2023-07-24",
            "target": "2023-09-07",
            "office": "Engineer's Office",
            "assigned": null,
            "appropriation": 400000,
            "abc": 0,
            "status_id": {
              "data": null
            },
            "remarks": "\"* 100% physically completed based on approved technical plans & specifications\n* w/ time extension as per approved SO No.1\n* Adjusted Targe Date of Completion: Oct. 19, 2023\"\n* Actua Comp[letion Date: 10/16/2023\n",
            "entry_type": "IMPLEMENTED",
            "bid_date": null,
            "year": 2023,
            "abc_published": null,
            "prop_takers": "KAG.VEDRA/ANGGO",
            "prop_infra": null,
            "time_extension": null
          }
        }), db_connection).await,
convert_projects_into_active_model_v2(&json!({
          "id": "1102",
          "attributes": {
            "name": "Concreting of Road at Prk Rosas to Prk  Bombil, Barangay Bulawan\n",
            "code": "1SAIL23 - 8000 - A14",
            "location": "[\"25\"]",
            "cost": 1736685,
            "contractor": {
              "data": {
                "id": "50",
                "attributes": {
                  "name": "Diplahan Construction",
                  "street_purok": null
                }
              }
            },
            "duration": 0,
            "adjusted": null,
            "total": null,
            "weight": null,
            "prop_sdg": "[]",
            "prop_fund": "19",
            "prop_type": "1",
            "prop_category": "2",
            "prop_sector": "[]",
            "prop_assign": "2",
            "accom_total": 80,
            "accom_value": null,
            "accom_date": null,
            "accom_elapse": null,
            "accom_slippage": null,
            "accom_period": null,
            "accom_todate": null,
            "accom_qty": null,
            "accom_amt": null,
            "date": null,
            "status": "CREATED",
            "start": "2023-06-14",
            "target": null,
            "office": "Engineer's Office",
            "assigned": null,
            "appropriation": 1800000,
            "abc": 0,
            "status_id": {
              "data": null
            },
            "remarks": "80% physical accomplishment as of Oct. 21, 2023\n\n90% physical accomplishment as of Oct. 27, 2023",
            "entry_type": "IMPLEMENTED",
            "bid_date": null,
            "year": 2023,
            "abc_published": null,
            "prop_takers": null,
            "prop_infra": null,
            "time_extension": null
          }
        }), db_connection).await,
convert_projects_into_active_model_v2(&json!({
          "id": "1103",
          "attributes": {
            "name": "Concreting of Road Leading to Senior Citizen Office, Barangay Danlugan",
            "code": "1SAIL23-8000-A15",
            "location": "[\"27\"]",
            "cost": 960311.84,
            "contractor": {
              "data": {
                "id": "6",
                "attributes": {
                  "name": "DM Ventures",
                  "street_purok": null
                }
              }
            },
            "duration": 90,
            "adjusted": null,
            "total": null,
            "weight": null,
            "prop_sdg": "[]",
            "prop_fund": "19",
            "prop_type": "1",
            "prop_category": "2",
            "prop_sector": "[]",
            "prop_assign": "2",
            "accom_total": 100,
            "accom_value": null,
            "accom_date": null,
            "accom_elapse": null,
            "accom_slippage": null,
            "accom_period": null,
            "accom_todate": null,
            "accom_qty": null,
            "accom_amt": null,
            "date": null,
            "status": "CREATED",
            "start": "2023-07-10",
            "target": "2023-10-08",
            "office": "Engineer's Office",
            "assigned": null,
            "appropriation": 1000000,
            "abc": 961347.66,
            "status_id": {
              "data": null
            },
            "remarks": "100% physically completed based on approved technical plans & specifications\n",
            "entry_type": "IMPLEMENTED",
            "bid_date": null,
            "year": 2023,
            "abc_published": null,
            "prop_takers": "CAPT. DACAL",
            "prop_infra": null,
            "time_extension": null
          }
        }), db_connection).await,
convert_projects_into_active_model_v2(&json!({
          "id": "1104",
          "attributes": {
            "name": "Road Widening Leading to Pagadian City NHS, Barangay Danlugan",
            "code": "1SAIL23-8000-A16",
            "location": "[\"27\"]",
            "cost": 484868.94,
            "contractor": {
              "data": {
                "id": "6",
                "attributes": {
                  "name": "DM Ventures",
                  "street_purok": null
                }
              }
            },
            "duration": 70,
            "adjusted": null,
            "total": null,
            "weight": null,
            "prop_sdg": "[]",
            "prop_fund": "19",
            "prop_type": "1",
            "prop_category": "2",
            "prop_sector": "[]",
            "prop_assign": "2",
            "accom_total": 100,
            "accom_value": null,
            "accom_date": null,
            "accom_elapse": null,
            "accom_slippage": null,
            "accom_period": null,
            "accom_todate": null,
            "accom_qty": null,
            "accom_amt": null,
            "date": null,
            "status": "CREATED",
            "start": "2023-09-21",
            "target": "2023-11-30",
            "office": "Engineer's Office",
            "assigned": null,
            "appropriation": 500000,
            "abc": 484868.94,
            "status_id": {
              "data": null
            },
            "remarks": "100% physically completed based on approved technical plans & specifications\n",
            "entry_type": "IMPLEMENTED",
            "bid_date": null,
            "year": 2023,
            "abc_published": null,
            "prop_takers": "CAPT. DACAL",
            "prop_infra": null,
            "time_extension": null
          }
        }), db_connection).await,
convert_projects_into_active_model_v2(&json!({
          "id": "1105",
          "attributes": {
            "name": "Concreting of Road at Prk Lusaran, Barangay Dao\n",
            "code": "1SAIL23-8000-A17",
            "location": "[\"5\"]",
            "cost": 577907.59,
            "contractor": {
              "data": {
                "id": "19",
                "attributes": {
                  "name": "Jenrich Construction",
                  "street_purok": null
                }
              }
            },
            "duration": 80,
            "adjusted": null,
            "total": null,
            "weight": null,
            "prop_sdg": "[]",
            "prop_fund": "19",
            "prop_type": "1",
            "prop_category": "2",
            "prop_sector": "[]",
            "prop_assign": "2",
            "accom_total": 100,
            "accom_value": null,
            "accom_date": null,
            "accom_elapse": null,
            "accom_slippage": null,
            "accom_period": null,
            "accom_todate": null,
            "accom_qty": null,
            "accom_amt": null,
            "date": null,
            "status": "CREATED",
            "start": "2023-09-04",
            "target": "2023-11-23",
            "office": "Engineer's Office",
            "assigned": null,
            "appropriation": 600000,
            "abc": 0,
            "status_id": {
              "data": null
            },
            "remarks": "\"* 100% physically completed base on approved technical plans & specifications\n* w/ time extension as per approved Suspension Order No. 1\n* Adjusted Targe Date of Completion: Dec. 24, 2023\"\n* Actual Completion Date: 12/23/2023",
            "entry_type": "IMPLEMENTED",
            "bid_date": null,
            "year": 2023,
            "abc_published": null,
            "prop_takers": "\" KAG. ORDENIZA-BOGO 09487176107\"",
            "prop_infra": null,
            "time_extension": null
          }
        }), db_connection).await,
convert_projects_into_active_model_v2(&json!({
          "id": "1106",
          "attributes": {
            "name": "Concreting of Road at Prk Patag, Barangay Dao\n",
            "code": "1SAIL23-8000-A18",
            "location": "[\"5\"]",
            "cost": 288986.55,
            "contractor": {
              "data": {
                "id": "1",
                "attributes": {
                  "name": "CTG Construction",
                  "street_purok": "Purok 1"
                }
              }
            },
            "duration": 45,
            "adjusted": null,
            "total": null,
            "weight": null,
            "prop_sdg": "[]",
            "prop_fund": "19",
            "prop_type": "1",
            "prop_category": "2",
            "prop_sector": "[]",
            "prop_assign": "2",
            "accom_total": 100,
            "accom_value": null,
            "accom_date": null,
            "accom_elapse": null,
            "accom_slippage": null,
            "accom_period": null,
            "accom_todate": null,
            "accom_qty": null,
            "accom_amt": null,
            "date": null,
            "status": "CREATED",
            "start": "2023-07-10",
            "target": "2023-08-24",
            "office": "Engineer's Office",
            "assigned": null,
            "appropriation": 300000,
            "abc": 0,
            "status_id": {
              "data": null
            },
            "remarks": "\"* 100% physically completed based on approved technical plans & specifications\n* w/ time extension as per approved SO No.1 \n* Adjusted Targe Date of Completion: Nov. 20, 2023\"\n* Actual Completion Date:11/20/2023",
            "entry_type": "IMPLEMENTED",
            "bid_date": null,
            "year": 2023,
            "abc_published": null,
            "prop_takers": "PARAGUYA REMIL",
            "prop_infra": null,
            "time_extension": null
          }
        }), db_connection).await,
convert_projects_into_active_model_v2(&json!({
          "id": "1107",
          "attributes": {
            "name": "Concreting of Road at Prk Malipayon (From New Concreted Main Road), Barangay Dao\n",
            "code": "1SAIL23-8000-A19",
            "location": "[\"5\"]",
            "cost": 0,
            "contractor": {
              "data": {
                "id": "19",
                "attributes": {
                  "name": "Jenrich Construction",
                  "street_purok": null
                }
              }
            },
            "duration": 90,
            "adjusted": null,
            "total": null,
            "weight": null,
            "prop_sdg": "[]",
            "prop_fund": "19",
            "prop_type": "1",
            "prop_category": "2",
            "prop_sector": "[]",
            "prop_assign": "2",
            "accom_total": 70,
            "accom_value": null,
            "accom_date": null,
            "accom_elapse": null,
            "accom_slippage": null,
            "accom_period": null,
            "accom_todate": null,
            "accom_qty": null,
            "accom_amt": null,
            "date": null,
            "status": "CREATED",
            "start": "2024-01-03",
            "target": "2024-04-02",
            "office": "Engineer's Office",
            "assigned": null,
            "appropriation": 500000,
            "abc": 0,
            "status_id": {
              "data": null
            },
            "remarks": "On-going implementation of the project w/ 70% physical accomplishment as of March 8, 2024",
            "entry_type": "IMPLEMENTED",
            "bid_date": null,
            "year": 2023,
            "abc_published": null,
            "prop_takers": "\" SUSAN BEARMINO 09505651740 BONTILAO\"",
            "prop_infra": null,
            "time_extension": null
          }
        }), db_connection).await,
convert_projects_into_active_model_v2(&json!({
          "id": "1108",
          "attributes": {
            "name": "Gravelling of Road at Prk 4 & Prk 5, Barangay Deborok \n",
            "code": "1SAIL23 - 8000 - A20",
            "location": "[\"29\"]",
            "cost": 483832.5,
            "contractor": {
              "data": {
                "id": "30",
                "attributes": {
                  "name": "M&L Construction",
                  "street_purok": null
                }
              }
            },
            "duration": null,
            "adjusted": null,
            "total": null,
            "weight": null,
            "prop_sdg": "[]",
            "prop_fund": "19",
            "prop_type": "1",
            "prop_category": "4",
            "prop_sector": "[]",
            "prop_assign": "2",
            "accom_total": 0,
            "accom_value": null,
            "accom_date": null,
            "accom_elapse": null,
            "accom_slippage": null,
            "accom_period": null,
            "accom_todate": null,
            "accom_qty": null,
            "accom_amt": null,
            "date": null,
            "status": "CREATED",
            "start": "",
            "target": null,
            "office": "Engineer's Office",
            "assigned": null,
            "appropriation": 500000,
            "abc": 0,
            "status_id": {
              "data": null
            },
            "remarks": null,
            "entry_type": "IMPLEMENTED",
            "bid_date": null,
            "year": 2023,
            "abc_published": null,
            "prop_takers": null,
            "prop_infra": null,
            "time_extension": null
          }
        }), db_connection).await,
convert_projects_into_active_model_v2(&json!({
          "id": "1109",
          "attributes": {
            "name": "Gravelling of Road at Prk3, Barangay Deborok \n",
            "code": "1SAIL23 - 8000 - A21",
            "location": "[\"29\"]",
            "cost": 0,
            "contractor": {
              "data": null
            },
            "duration": null,
            "adjusted": null,
            "total": null,
            "weight": null,
            "prop_sdg": "[]",
            "prop_fund": "19",
            "prop_type": null,
            "prop_category": null,
            "prop_sector": "[]",
            "prop_assign": null,
            "accom_total": 0,
            "accom_value": null,
            "accom_date": null,
            "accom_elapse": null,
            "accom_slippage": null,
            "accom_period": null,
            "accom_todate": null,
            "accom_qty": null,
            "accom_amt": null,
            "date": null,
            "status": "CREATED",
            "start": null,
            "target": null,
            "office": "Engineer's Office",
            "assigned": null,
            "appropriation": 500000,
            "abc": 0,
            "status_id": {
              "data": null
            },
            "remarks": null,
            "entry_type": "IMPLEMENTED",
            "bid_date": null,
            "year": 2023,
            "abc_published": null,
            "prop_takers": null,
            "prop_infra": null,
            "time_extension": null
          }
        }), db_connection).await,
        convert_projects_into_active_model_v2(&json!({
          "id": "1110",
          "attributes": {
            "name": "Continuation of Widening of Road at Prk Bagong Silao, Barangay Ditoray\n",
            "code": "1SAIL23 - 8000 - A22",
            "location": "[\"30\"]",
            "cost": 0,
            "contractor": {
              "data": null
            },
            "duration": null,
            "adjusted": null,
            "total": null,
            "weight": null,
            "prop_sdg": "[]",
            "prop_fund": "19",
            "prop_type": null,
            "prop_category": null,
            "prop_sector": "[]",
            "prop_assign": null,
            "accom_total": 0,
            "accom_value": null,
            "accom_date": null,
            "accom_elapse": null,
            "accom_slippage": null,
            "accom_period": null,
            "accom_todate": null,
            "accom_qty": null,
            "accom_amt": null,
            "date": null,
            "status": "CREATED",
            "start": null,
            "target": null,
            "office": "Engineer's Office",
            "assigned": null,
            "appropriation": 1400000,
            "abc": 0,
            "status_id": {
              "data": null
            },
            "remarks": null,
            "entry_type": "IMPLEMENTED",
            "bid_date": null,
            "year": 2023,
            "abc_published": null,
            "prop_takers": null,
            "prop_infra": null,
            "time_extension": null
          }
        }), db_connection).await,

convert_projects_into_active_model_v2(&json!({
          "id": "1111",
          "attributes": {
            "name": "Concreting of Road Leading to Elem School Site, Barangay Gubac\n",
            "code": "1SAIL23 - 8000 - A23",
            "location": "[\"31\"]",
            "cost": 482569.1,
            "contractor": {
              "data": {
                "id": "19",
                "attributes": {
                  "name": "Jenrich Construction",
                  "street_purok": null
                }
              }
            },
            "duration": null,
            "adjusted": null,
            "total": null,
            "weight": null,
            "prop_sdg": "[]",
            "prop_fund": "19",
            "prop_type": "1",
            "prop_category": "2",
            "prop_sector": "[]",
            "prop_assign": "2",
            "accom_total": 0,
            "accom_value": null,
            "accom_date": null,
            "accom_elapse": null,
            "accom_slippage": null,
            "accom_period": null,
            "accom_todate": null,
            "accom_qty": null,
            "accom_amt": null,
            "date": null,
            "status": "CREATED",
            "start": null,
            "target": null,
            "office": "Engineer's Office",
            "assigned": null,
            "appropriation": 500000,
            "abc": 0,
            "status_id": {
              "data": null
            },
            "remarks": null,
            "entry_type": "IMPLEMENTED",
            "bid_date": null,
            "year": 2023,
            "abc_published": null,
            "prop_takers": null,
            "prop_infra": null,
            "time_extension": null
          }
        }), db_connection).await,

convert_projects_into_active_model_v2(&json!({
          "id": "1112",
          "attributes": {
            "name": "Concreting of Road at Upper Manguilles, Barangay Gubang\n",
            "code": "1SAIL23-8000-A24",
            "location": "[]",
            "cost": 960568,
            "contractor": {
              "data": {
                "id": "50",
                "attributes": {
                  "name": "Diplahan Construction",
                  "street_purok": null
                }
              }
            },
            "duration": 45,
            "adjusted": null,
            "total": null,
            "weight": null,
            "prop_sdg": "[]",
            "prop_fund": "19",
            "prop_type": "1",
            "prop_category": "2",
            "prop_sector": "[]",
            "prop_assign": "2",
            "accom_total": 100,
            "accom_value": null,
            "accom_date": null,
            "accom_elapse": null,
            "accom_slippage": null,
            "accom_period": null,
            "accom_todate": null,
            "accom_qty": null,
            "accom_amt": null,
            "date": null,
            "status": "CREATED",
            "start": "2023-07-17",
            "target": "2023-08-31",
            "office": "Engineer's Office",
            "assigned": null,
            "appropriation": 1000000,
            "abc": 0,
            "status_id": {
              "data": null
            },
            "remarks": "100% physically completed based on approved technical plans & specifications\n\nActual date Completed: 9/1/2023\n",
            "entry_type": "IMPLEMENTED",
            "bid_date": null,
            "year": 2023,
            "abc_published": null,
            "prop_takers": "\" CAPT. MANGOY KAG TEJADA\"",
            "prop_infra": null,
            "time_extension": null
          }
        }), db_connection).await,

convert_projects_into_active_model_v2(&json!({
          "id": "1113",
          "attributes": {
            "name": "Concreting of Road at Prk Waling-Waling (Phase II) Barangay Kagawasan",
            "code": "1SAIL23-8000-A25",
            "location": "[\"33\"]",
            "cost": 675974.46,
            "contractor": {
              "data": {
                "id": "10",
                "attributes": {
                  "name": "RSQ Construction",
                  "street_purok": null
                }
              }
            },
            "duration": 19966,
            "adjusted": null,
            "total": null,
            "weight": null,
            "prop_sdg": "[]",
            "prop_fund": "19",
            "prop_type": "1",
            "prop_category": "2",
            "prop_sector": "[]",
            "prop_assign": "2",
            "accom_total": 90,
            "accom_value": null,
            "accom_date": null,
            "accom_elapse": null,
            "accom_slippage": null,
            "accom_period": null,
            "accom_todate": null,
            "accom_qty": null,
            "accom_amt": null,
            "date": null,
            "status": "CREATED",
            "start": "Invalid date",
            "target": "2024-08-30",
            "office": "Engineer's Office",
            "assigned": null,
            "appropriation": 700000,
            "abc": 676675.12,
            "status_id": {
              "data": null
            },
            "remarks": "On-going w/ a physical accomplishment of 90% as of 8/2/2024\n",
            "entry_type": "IMPLEMENTED",
            "bid_date": null,
            "year": 2023,
            "abc_published": null,
            "prop_takers": null,
            "prop_infra": null,
            "time_extension": null
          }
        }), db_connection).await,

convert_projects_into_active_model_v2(&json!({
          "id": "1114",
          "attributes": {
            "name": "Concreting of Road at Prk Rosas, Barangay Kagawasan\n",
            "code": "1SAIL23 - 8000 - A26",
            "location": "[\"33\"]",
            "cost": 773997,
            "contractor": {
              "data": {
                "id": "10",
                "attributes": {
                  "name": "RSQ Construction",
                  "street_purok": null
                }
              }
            },
            "duration": null,
            "adjusted": null,
            "total": null,
            "weight": null,
            "prop_sdg": "[]",
            "prop_fund": "19",
            "prop_type": "1",
            "prop_category": "2",
            "prop_sector": "[]",
            "prop_assign": "2",
            "accom_total": 35,
            "accom_value": null,
            "accom_date": null,
            "accom_elapse": null,
            "accom_slippage": null,
            "accom_period": null,
            "accom_todate": null,
            "accom_qty": null,
            "accom_amt": null,
            "date": null,
            "status": "CREATED",
            "start": null,
            "target": null,
            "office": "Engineer's Office",
            "assigned": null,
            "appropriation": 800000,
            "abc": 0,
            "status_id": {
              "data": null
            },
            "remarks": "35% physical accomplishment as of March 8, 2024",
            "entry_type": "IMPLEMENTED",
            "bid_date": null,
            "year": 2023,
            "abc_published": null,
            "prop_takers": null,
            "prop_infra": null,
            "time_extension": null
          }
        }), db_connection).await,

convert_projects_into_active_model_v2(&json!({
          "id": "1115",
          "attributes": {
            "name": "Concreting of Road at Prk Talong to Prk Balatong Uno (Phase III), Barangay Kahayagan",
            "code": "1SAIP23 - 8000 - A27",
            "location": "[\"33\"]",
            "cost": 967543.6,
            "contractor": {
              "data": {
                "id": "1",
                "attributes": {
                  "name": "CTG Construction",
                  "street_purok": "Purok 1"
                }
              }
            },
            "duration": null,
            "adjusted": null,
            "total": null,
            "weight": null,
            "prop_sdg": "[]",
            "prop_fund": "19",
            "prop_type": "1",
            "prop_category": "2",
            "prop_sector": "[]",
            "prop_assign": "2",
            "accom_total": 5,
            "accom_value": null,
            "accom_date": null,
            "accom_elapse": null,
            "accom_slippage": null,
            "accom_period": null,
            "accom_todate": null,
            "accom_qty": null,
            "accom_amt": null,
            "date": null,
            "status": "CREATED",
            "start": null,
            "target": null,
            "office": "Engineer's Office",
            "assigned": null,
            "appropriation": 1000000,
            "abc": 0,
            "status_id": {
              "data": null
            },
            "remarks": null,
            "entry_type": "IMPLEMENTED",
            "bid_date": null,
            "year": 2023,
            "abc_published": null,
            "prop_takers": null,
            "prop_infra": null,
            "time_extension": null
          }
        }), db_connection).await,

convert_projects_into_active_model_v2(&json!({
          "id": "1116",
          "attributes": {
            "name": "Road Gravelling at Prk Talong 1 to Prk Kamatis, Barangay Kahayagan\n",
            "code": "1SAIL23 - 8000 - A28",
            "location": "[\"34\"]",
            "cost": 0,
            "contractor": {
              "data": null
            },
            "duration": null,
            "adjusted": null,
            "total": null,
            "weight": null,
            "prop_sdg": "[]",
            "prop_fund": "5",
            "prop_type": null,
            "prop_category": null,
            "prop_sector": "[]",
            "prop_assign": null,
            "accom_total": 0,
            "accom_value": null,
            "accom_date": null,
            "accom_elapse": null,
            "accom_slippage": null,
            "accom_period": null,
            "accom_todate": null,
            "accom_qty": null,
            "accom_amt": null,
            "date": null,
            "status": "CREATED",
            "start": null,
            "target": null,
            "office": "Engineer's Office",
            "assigned": null,
            "appropriation": 400000,
            "abc": 0,
            "status_id": {
              "data": null
            },
            "remarks": null,
            "entry_type": "IMPLEMENTED",
            "bid_date": null,
            "year": 2023,
            "abc_published": null,
            "prop_takers": null,
            "prop_infra": null,
            "time_extension": null
          }
        }), db_connection).await,

convert_projects_into_active_model_v2(&json!({
          "id": "1117",
          "attributes": {
            "name": "Concreting of Road w/RCCP Installation at Prk 6, Kalasan\n",
            "code": "1SAIL23-8000-A29",
            "location": "[\"35\"]",
            "cost": 965365.7,
            "contractor": {
              "data": {
                "id": "30",
                "attributes": {
                  "name": "M&L Construction",
                  "street_purok": null
                }
              }
            },
            "duration": 60,
            "adjusted": null,
            "total": null,
            "weight": null,
            "prop_sdg": "[]",
            "prop_fund": "19",
            "prop_type": "1",
            "prop_category": "2",
            "prop_sector": "[]",
            "prop_assign": "2",
            "accom_total": 100,
            "accom_value": null,
            "accom_date": null,
            "accom_elapse": null,
            "accom_slippage": null,
            "accom_period": null,
            "accom_todate": null,
            "accom_qty": null,
            "accom_amt": null,
            "date": null,
            "status": "CREATED",
            "start": "2023-10-02",
            "target": "2023-12-01",
            "office": "Engineer's Office",
            "assigned": null,
            "appropriation": 1000000,
            "abc": 0,
            "status_id": {
              "data": null
            },
            "remarks": "100% physically completed based on approved technical plans & specifications\n",
            "entry_type": "IMPLEMENTED",
            "bid_date": null,
            "year": 2023,
            "abc_published": null,
            "prop_takers": "LYNETTE PERALTA",
            "prop_infra": null,
            "time_extension": null
          }
        }), db_connection).await,

convert_projects_into_active_model_v2(&json!({
          "id": "1118",
          "attributes": {
            "name": "Gravelling of Road from Prk 4 to Prk 1, Kalasan\n",
            "code": "1SAIL23 - 8000 - A30",
            "location": "[\"35\"]",
            "cost": 432681,
            "contractor": {
              "data": {
                "id": "1",
                "attributes": {
                  "name": "CTG Construction",
                  "street_purok": "Purok 1"
                }
              }
            },
            "duration": null,
            "adjusted": null,
            "total": null,
            "weight": null,
            "prop_sdg": "[]",
            "prop_fund": "19",
            "prop_type": "1",
            "prop_category": "4",
            "prop_sector": "[]",
            "prop_assign": "2",
            "accom_total": 0,
            "accom_value": null,
            "accom_date": null,
            "accom_elapse": null,
            "accom_slippage": null,
            "accom_period": null,
            "accom_todate": null,
            "accom_qty": null,
            "accom_amt": null,
            "date": null,
            "status": "CREATED",
            "start": null,
            "target": null,
            "office": "Engineer's Office",
            "assigned": null,
            "appropriation": 450000,
            "abc": 0,
            "status_id": {
              "data": null
            },
            "remarks": null,
            "entry_type": "IMPLEMENTED",
            "bid_date": null,
            "year": 2023,
            "abc_published": null,
            "prop_takers": null,
            "prop_infra": null,
            "time_extension": null
          }
        }), db_connection).await,

convert_projects_into_active_model_v2(&json!({
          "id": "1119",
          "attributes": {
            "name": "Concreting of Road Leading to Elem School Site, Barangay Kawit\n",
            "code": "1SAIL23-8000-A31",
            "location": "[\"8\"]",
            "cost": 285227.46,
            "contractor": {
              "data": {
                "id": "5",
                "attributes": {
                  "name": "MOLROW CONSTRUCTION",
                  "street_purok": null
                }
              }
            },
            "duration": 45,
            "adjusted": null,
            "total": null,
            "weight": null,
            "prop_sdg": "[]",
            "prop_fund": "19",
            "prop_type": "1",
            "prop_category": "2",
            "prop_sector": "[]",
            "prop_assign": "2",
            "accom_total": 100,
            "accom_value": null,
            "accom_date": null,
            "accom_elapse": null,
            "accom_slippage": null,
            "accom_period": null,
            "accom_todate": null,
            "accom_qty": null,
            "accom_amt": null,
            "date": null,
            "status": "CREATED",
            "start": "2023-08-28",
            "target": "2023-10-12",
            "office": "Engineer's Office",
            "assigned": null,
            "appropriation": 300000,
            "abc": 290086.21,
            "status_id": {
              "data": null
            },
            "remarks": "\"* 100% physically completed base on approved technical plans & specifications\n* Adjusted Target Date of Completion: March 22, 2024 as per Suspension Order No.1\n* Actual Completion Date: March 22, 2024\"\n",
            "entry_type": "IMPLEMENTED",
            "bid_date": null,
            "year": 2023,
            "abc_published": null,
            "prop_takers": "CAPT. DIMASAR",
            "prop_infra": null,
            "time_extension": null
          }
        }), db_connection).await,
        convert_projects_into_active_model_v2(&json!({
          "id": "1120",
          "attributes": {
            "name": "Concreting of Road at Prk Muslim Village (Continuation), Barangay Kawit\n",
            "code": "1SAIL23-8000-A32",
            "location": "[\"8\"]",
            "cost": 936600,
            "contractor": {
              "data": {
                "id": "5",
                "attributes": {
                  "name": "MOLROW CONSTRUCTION",
                  "street_purok": null
                }
              }
            },
            "duration": 90,
            "adjusted": null,
            "total": null,
            "weight": null,
            "prop_sdg": "[]",
            "prop_fund": "19",
            "prop_type": "1",
            "prop_category": "2",
            "prop_sector": "[]",
            "prop_assign": "2",
            "accom_total": 100,
            "accom_value": null,
            "accom_date": null,
            "accom_elapse": null,
            "accom_slippage": null,
            "accom_period": null,
            "accom_todate": null,
            "accom_qty": null,
            "accom_amt": null,
            "date": null,
            "status": "CREATED",
            "start": "2023-08-07",
            "target": "2023-11-05",
            "office": "Engineer's Office",
            "assigned": null,
            "appropriation": 1000000,
            "abc": 966782.01,
            "status_id": {
              "data": null
            },
            "remarks": "\"* 100% physically completed base on approved technical plans & specifications\n* w/ time extension as per approved Variation  Order No. 1\n* Adjusted Targe Date of Completion: 1/7/2024\"\n",
            "entry_type": "IMPLEMENTED",
            "bid_date": null,
            "year": 2023,
            "abc_published": null,
            "prop_takers": null,
            "prop_infra": null,
            "time_extension": null
          }
        }), db_connection).await,
convert_projects_into_active_model_v2(&json!({
          "id": "1121",
          "attributes": {
            "name": "Concreting of Road at Prk Pinya to Prk Santol, Barangay La Suerte\n",
            "code": "1SAIL23 - 8000 - A33",
            "location": "[\"36\"]",
            "cost": 967999.7,
            "contractor": {
              "data": {
                "id": "19",
                "attributes": {
                  "name": "Jenrich Construction",
                  "street_purok": null
                }
              }
            },
            "duration": null,
            "adjusted": null,
            "total": null,
            "weight": null,
            "prop_sdg": "[]",
            "prop_fund": "19",
            "prop_type": "1",
            "prop_category": "2",
            "prop_sector": "[]",
            "prop_assign": "2",
            "accom_total": 60,
            "accom_value": null,
            "accom_date": null,
            "accom_elapse": null,
            "accom_slippage": null,
            "accom_period": null,
            "accom_todate": null,
            "accom_qty": null,
            "accom_amt": null,
            "date": null,
            "status": "CREATED",
            "start": null,
            "target": null,
            "office": "Engineer's Office",
            "assigned": null,
            "appropriation": 1000000,
            "abc": 0,
            "status_id": {
              "data": null
            },
            "remarks": "60% physical accomplishment as of oct. 21, 2023\n\n85% physical accomplishment as of oct. 27, 2023",
            "entry_type": "IMPLEMENTED",
            "bid_date": null,
            "year": 2023,
            "abc_published": null,
            "prop_takers": null,
            "prop_infra": null,
            "time_extension": null
          }
        }), db_connection).await,
convert_projects_into_active_model_v2(&json!({
          "id": "1122",
          "attributes": {
            "name": "Gravelling of Road at Prk Pinya - Prk Lubi, Barangay La Suerte\n",
            "code": "1SAIL23 - 8000 - A34",
            "location": "[\"36\"]",
            "cost": 386222.6,
            "contractor": {
              "data": {
                "id": "19",
                "attributes": {
                  "name": "Jenrich Construction",
                  "street_purok": null
                }
              }
            },
            "duration": null,
            "adjusted": null,
            "total": null,
            "weight": null,
            "prop_sdg": "[]",
            "prop_fund": "19",
            "prop_type": "1",
            "prop_category": "4",
            "prop_sector": "[]",
            "prop_assign": "2",
            "accom_total": 0,
            "accom_value": null,
            "accom_date": null,
            "accom_elapse": null,
            "accom_slippage": null,
            "accom_period": null,
            "accom_todate": null,
            "accom_qty": null,
            "accom_amt": null,
            "date": null,
            "status": "CREATED",
            "start": null,
            "target": null,
            "office": "Engineer's Office",
            "assigned": null,
            "appropriation": 400000,
            "abc": 0,
            "status_id": {
              "data": null
            },
            "remarks": null,
            "entry_type": "IMPLEMENTED",
            "bid_date": null,
            "year": 2023,
            "abc_published": null,
            "prop_takers": null,
            "prop_infra": null,
            "time_extension": null
          }
        }), db_connection).await,
convert_projects_into_active_model_v2(&json!({
          "id": "1123",
          "attributes": {
            "name": "Concreting of Road From Brgy Lala to Lumad (Continuation, Barangay Lala\n",
            "code": "1SAIL23 - 8000 - A35",
            "location": "[\"37\"]",
            "cost": 967500,
            "contractor": {
              "data": {
                "id": "48",
                "attributes": {
                  "name": "AFS Construction",
                  "street_purok": null
                }
              }
            },
            "duration": null,
            "adjusted": null,
            "total": null,
            "weight": null,
            "prop_sdg": "[]",
            "prop_fund": "19",
            "prop_type": "1",
            "prop_category": "2",
            "prop_sector": "[]",
            "prop_assign": "2",
            "accom_total": 100,
            "accom_value": null,
            "accom_date": null,
            "accom_elapse": null,
            "accom_slippage": null,
            "accom_period": null,
            "accom_todate": null,
            "accom_qty": null,
            "accom_amt": null,
            "date": null,
            "status": "CREATED",
            "start": null,
            "target": null,
            "office": "Engineer's Office",
            "assigned": null,
            "appropriation": 1000000,
            "abc": 0,
            "status_id": {
              "data": null
            },
            "remarks": null,
            "entry_type": "IMPLEMENTED",
            "bid_date": null,
            "year": 2023,
            "abc_published": null,
            "prop_takers": null,
            "prop_infra": null,
            "time_extension": null
          }
        }), db_connection).await,
convert_projects_into_active_model_v2(&json!({
          "id": "1124",
          "attributes": {
            "name": "Concreting of Road Purok Masignutanon Alley, Barangay Lala\n",
            "code": "1SAIL23 - 8000 - A36",
            "location": "[\"37\"]",
            "cost": 0,
            "contractor": {
              "data": null
            },
            "duration": null,
            "adjusted": null,
            "total": null,
            "weight": null,
            "prop_sdg": "[]",
            "prop_fund": "5",
            "prop_type": null,
            "prop_category": null,
            "prop_sector": "[]",
            "prop_assign": null,
            "accom_total": 0,
            "accom_value": null,
            "accom_date": null,
            "accom_elapse": null,
            "accom_slippage": null,
            "accom_period": null,
            "accom_todate": null,
            "accom_qty": null,
            "accom_amt": null,
            "date": null,
            "status": "CREATED",
            "start": null,
            "target": null,
            "office": "Engineer's Office",
            "assigned": null,
            "appropriation": 400000,
            "abc": 0,
            "status_id": {
              "data": null
            },
            "remarks": null,
            "entry_type": "IMPLEMENTED",
            "bid_date": null,
            "year": 2023,
            "abc_published": null,
            "prop_takers": null,
            "prop_infra": null,
            "time_extension": null
          }
        }), db_connection).await,
convert_projects_into_active_model_v2(&json!({
          "id": "1125",
          "attributes": {
            "name": "Concreting of Road Purok Masanagon Alley, Barangay Lala\n",
            "code": "1SAIL23 - 8000 - A37",
            "location": "[\"37\"]",
            "cost": 385990.58,
            "contractor": {
              "data": {
                "id": "1",
                "attributes": {
                  "name": "CTG Construction",
                  "street_purok": "Purok 1"
                }
              }
            },
            "duration": null,
            "adjusted": null,
            "total": null,
            "weight": null,
            "prop_sdg": "[]",
            "prop_fund": "19",
            "prop_type": "1",
            "prop_category": "2",
            "prop_sector": "[]",
            "prop_assign": "2",
            "accom_total": 0,
            "accom_value": null,
            "accom_date": null,
            "accom_elapse": null,
            "accom_slippage": null,
            "accom_period": null,
            "accom_todate": null,
            "accom_qty": null,
            "accom_amt": null,
            "date": null,
            "status": "CREATED",
            "start": null,
            "target": null,
            "office": "Engineer's Office",
            "assigned": null,
            "appropriation": 400000,
            "abc": 0,
            "status_id": {
              "data": null
            },
            "remarks": null,
            "entry_type": "IMPLEMENTED",
            "bid_date": null,
            "year": 2023,
            "abc_published": null,
            "prop_takers": null,
            "prop_infra": null,
            "time_extension": null
          }
        }), db_connection).await,
convert_projects_into_active_model_v2(&json!({
          "id": "1126",
          "attributes": {
            "name": "Concreting/Improvement of Road Purok Sandayong (Continuation), Barangay Lapedian\n",
            "code": "1SAIL23 - 8000 - A38",
            "location": "[\"38\"]",
            "cost": 481996,
            "contractor": {
              "data": {
                "id": "1",
                "attributes": {
                  "name": "CTG Construction",
                  "street_purok": "Purok 1"
                }
              }
            },
            "duration": null,
            "adjusted": null,
            "total": null,
            "weight": null,
            "prop_sdg": "[]",
            "prop_fund": "19",
            "prop_type": "1",
            "prop_category": "4",
            "prop_sector": "[]",
            "prop_assign": "2",
            "accom_total": 0,
            "accom_value": null,
            "accom_date": null,
            "accom_elapse": null,
            "accom_slippage": null,
            "accom_period": null,
            "accom_todate": null,
            "accom_qty": null,
            "accom_amt": null,
            "date": null,
            "status": "CREATED",
            "start": null,
            "target": null,
            "office": "Engineer's Office",
            "assigned": null,
            "appropriation": 500000,
            "abc": 0,
            "status_id": {
              "data": null
            },
            "remarks": null,
            "entry_type": "IMPLEMENTED",
            "bid_date": null,
            "year": 2023,
            "abc_published": null,
            "prop_takers": null,
            "prop_infra": null,
            "time_extension": null
          }
        }), db_connection).await,
convert_projects_into_active_model_v2(&json!({
          "id": "1127",
          "attributes": {
            "name": "Concreting of Road at Purok Kamansi, Barangay Lenienza\n",
            "code": "1SAIL23 - 8000 - A39",
            "location": "[\"39\"]",
            "cost": 579912.6,
            "contractor": {
              "data": {
                "id": "1",
                "attributes": {
                  "name": "CTG Construction",
                  "street_purok": "Purok 1"
                }
              }
            },
            "duration": null,
            "adjusted": null,
            "total": null,
            "weight": null,
            "prop_sdg": "[]",
            "prop_fund": "19",
            "prop_type": "1",
            "prop_category": "2",
            "prop_sector": "[]",
            "prop_assign": "2",
            "accom_total": 50,
            "accom_value": null,
            "accom_date": null,
            "accom_elapse": null,
            "accom_slippage": null,
            "accom_period": null,
            "accom_todate": null,
            "accom_qty": null,
            "accom_amt": null,
            "date": null,
            "status": "CREATED",
            "start": null,
            "target": null,
            "office": "Engineer's Office",
            "assigned": null,
            "appropriation": 600000,
            "abc": 0,
            "status_id": {
              "data": null
            },
            "remarks": null,
            "entry_type": "IMPLEMENTED",
            "bid_date": null,
            "year": 2023,
            "abc_published": null,
            "prop_takers": null,
            "prop_infra": null,
            "time_extension": null
          }
        }), db_connection).await,
convert_projects_into_active_model_v2(&json!({
          "id": "1128",
          "attributes": {
            "name": "Concreting of Road at Purok Mangga, Barangay Lenienza\n",
            "code": "1SAIL23 - 8000 - A40",
            "location": "[\"39\"]",
            "cost": 579912.6,
            "contractor": {
              "data": {
                "id": "1",
                "attributes": {
                  "name": "CTG Construction",
                  "street_purok": "Purok 1"
                }
              }
            },
            "duration": null,
            "adjusted": null,
            "total": null,
            "weight": null,
            "prop_sdg": "[]",
            "prop_fund": "19",
            "prop_type": "1",
            "prop_category": "2",
            "prop_sector": "[]",
            "prop_assign": "2",
            "accom_total": 0,
            "accom_value": null,
            "accom_date": null,
            "accom_elapse": null,
            "accom_slippage": null,
            "accom_period": null,
            "accom_todate": null,
            "accom_qty": null,
            "accom_amt": null,
            "date": null,
            "status": "CREATED",
            "start": null,
            "target": null,
            "office": "Engineer's Office",
            "assigned": null,
            "appropriation": 600000,
            "abc": 0,
            "status_id": {
              "data": null
            },
            "remarks": null,
            "entry_type": "IMPLEMENTED",
            "bid_date": null,
            "year": 2023,
            "abc_published": null,
            "prop_takers": null,
            "prop_infra": null,
            "time_extension": null
          }
        }), db_connection).await,
convert_projects_into_active_model_v2(&json!({
          "id": "1129",
          "attributes": {
            "name": "Concreting of Road Purok at Caimito, Barangay Lenienza\n",
            "code": "1SAIL23 - 8000 - A41",
            "location": "[\"39\"]",
            "cost": 0,
            "contractor": {
              "data": null
            },
            "duration": null,
            "adjusted": null,
            "total": null,
            "weight": null,
            "prop_sdg": "[]",
            "prop_fund": "19",
            "prop_type": "1",
            "prop_category": "2",
            "prop_sector": "[]",
            "prop_assign": "2",
            "accom_total": 50,
            "accom_value": null,
            "accom_date": null,
            "accom_elapse": null,
            "accom_slippage": null,
            "accom_period": null,
            "accom_todate": null,
            "accom_qty": null,
            "accom_amt": null,
            "date": null,
            "status": "CREATED",
            "start": null,
            "target": null,
            "office": "Engineer's Office",
            "assigned": null,
            "appropriation": 600000,
            "abc": 0,
            "status_id": {
              "data": null
            },
            "remarks": null,
            "entry_type": "IMPLEMENTED",
            "bid_date": null,
            "year": 2023,
            "abc_published": null,
            "prop_takers": null,
            "prop_infra": null,
            "time_extension": null
          }
        }), db_connection).await,
        convert_projects_into_active_model_v2(&json!({
          "id": "1130",
          "attributes": {
            "name": "Concreting/Improvement of Road at Purok Sirawag, Barangay Lison Valley\n",
            "code": "1SAIL23 - 8000 - A42",
            "location": "[\"40\"]",
            "cost": 240999,
            "contractor": {
              "data": {
                "id": "30",
                "attributes": {
                  "name": "M&L Construction",
                  "street_purok": null
                }
              }
            },
            "duration": null,
            "adjusted": null,
            "total": null,
            "weight": null,
            "prop_sdg": "[]",
            "prop_fund": "19",
            "prop_type": "1",
            "prop_category": "2",
            "prop_sector": "[]",
            "prop_assign": "2",
            "accom_total": 0,
            "accom_value": null,
            "accom_date": null,
            "accom_elapse": null,
            "accom_slippage": null,
            "accom_period": null,
            "accom_todate": null,
            "accom_qty": null,
            "accom_amt": null,
            "date": null,
            "status": "CREATED",
            "start": null,
            "target": null,
            "office": "Engineer's Office",
            "assigned": null,
            "appropriation": 250000,
            "abc": 0,
            "status_id": {
              "data": null
            },
            "remarks": null,
            "entry_type": "IMPLEMENTED",
            "bid_date": null,
            "year": 2023,
            "abc_published": null,
            "prop_takers": null,
            "prop_infra": null,
            "time_extension": null
          }
        }), db_connection).await,
convert_projects_into_active_model_v2(&json!({
          "id": "1131",
          "attributes": {
            "name": "Reblocking at Purok Santol (Near Lingue H/S) Barangay Lower Sibatang\n",
            "code": "1SAIL23 - 8000 - A43",
            "location": "[\"42\"]",
            "cost": 1161936.25,
            "contractor": {
              "data": {
                "id": "1",
                "attributes": {
                  "name": "CTG Construction",
                  "street_purok": "Purok 1"
                }
              }
            },
            "duration": null,
            "adjusted": null,
            "total": null,
            "weight": null,
            "prop_sdg": "[]",
            "prop_fund": "19",
            "prop_type": "1",
            "prop_category": "3",
            "prop_sector": "[]",
            "prop_assign": "2",
            "accom_total": 30,
            "accom_value": null,
            "accom_date": null,
            "accom_elapse": null,
            "accom_slippage": null,
            "accom_period": null,
            "accom_todate": null,
            "accom_qty": null,
            "accom_amt": null,
            "date": null,
            "status": "CREATED",
            "start": null,
            "target": null,
            "office": "Engineer's Office",
            "assigned": null,
            "appropriation": 1200000,
            "abc": 0,
            "status_id": {
              "data": null
            },
            "remarks": "30% physical accomplishment as of Nov. 10, 2023",
            "entry_type": "IMPLEMENTED",
            "bid_date": null,
            "year": 2023,
            "abc_published": null,
            "prop_takers": null,
            "prop_infra": null,
            "time_extension": null
          }
        }), db_connection).await,
convert_projects_into_active_model_v2(&json!({
          "id": "1132",
          "attributes": {
            "name": "Concreting of Road at Purok Kamunggay (Leading to Lala) Barangay Lumad\n",
            "code": "1SAIl23 - 8000 - A44",
            "location": "[\"43\"]",
            "cost": 0,
            "contractor": {
              "data": null
            },
            "duration": null,
            "adjusted": null,
            "total": null,
            "weight": null,
            "prop_sdg": "[]",
            "prop_fund": "19",
            "prop_type": "1",
            "prop_category": "2",
            "prop_sector": "[]",
            "prop_assign": "2",
            "accom_total": 0,
            "accom_value": null,
            "accom_date": null,
            "accom_elapse": null,
            "accom_slippage": null,
            "accom_period": null,
            "accom_todate": null,
            "accom_qty": null,
            "accom_amt": null,
            "date": null,
            "status": "CREATED",
            "start": null,
            "target": null,
            "office": "Engineer's Office",
            "assigned": null,
            "appropriation": 1800000,
            "abc": 0,
            "status_id": {
              "data": null
            },
            "remarks": null,
            "entry_type": "IMPLEMENTED",
            "bid_date": null,
            "year": 2023,
            "abc_published": null,
            "prop_takers": null,
            "prop_infra": null,
            "time_extension": null
          }
        }), db_connection).await,
convert_projects_into_active_model_v2(&json!({
          "id": "1133",
          "attributes": {
            "name": "Concreting of Road at Iglesia ni Kristo Church, Barangay Lumbia\n",
            "code": "1SAIL23 - 8000 - A45",
            "location": "[\"9\"]",
            "cost": 288012.34,
            "contractor": {
              "data": {
                "id": "19",
                "attributes": {
                  "name": "Jenrich Construction",
                  "street_purok": null
                }
              }
            },
            "duration": null,
            "adjusted": null,
            "total": null,
            "weight": null,
            "prop_sdg": "[]",
            "prop_fund": "19",
            "prop_type": "1",
            "prop_category": "2",
            "prop_sector": "[]",
            "prop_assign": "2",
            "accom_total": 100,
            "accom_value": null,
            "accom_date": null,
            "accom_elapse": null,
            "accom_slippage": null,
            "accom_period": null,
            "accom_todate": null,
            "accom_qty": null,
            "accom_amt": null,
            "date": null,
            "status": "CREATED",
            "start": null,
            "target": null,
            "office": "Engineer's Office",
            "assigned": null,
            "appropriation": 300000,
            "abc": 0,
            "status_id": {
              "data": null
            },
            "remarks": "100% physically completed as per report of project incharge",
            "entry_type": "IMPLEMENTED",
            "bid_date": null,
            "year": 2023,
            "abc_published": null,
            "prop_takers": null,
            "prop_infra": null,
            "time_extension": null
          }
        }), db_connection).await,
convert_projects_into_active_model_v2(&json!({
          "id": "1134",
          "attributes": {
            "name": " Improvement of Purok Lower Lumbia Road, Barangay Lumbia\n",
            "code": "1SAIL23 - 8000 - A46",
            "location": "[\"9\"]",
            "cost": 0,
            "contractor": {
              "data": null
            },
            "duration": null,
            "adjusted": null,
            "total": null,
            "weight": null,
            "prop_sdg": "[]",
            "prop_fund": "19",
            "prop_type": "1",
            "prop_category": "4",
            "prop_sector": "[]",
            "prop_assign": null,
            "accom_total": 0,
            "accom_value": null,
            "accom_date": null,
            "accom_elapse": null,
            "accom_slippage": null,
            "accom_period": null,
            "accom_todate": null,
            "accom_qty": null,
            "accom_amt": null,
            "date": null,
            "status": "CREATED",
            "start": null,
            "target": null,
            "office": "Engineer's Office",
            "assigned": null,
            "appropriation": 200000,
            "abc": 0,
            "status_id": {
              "data": null
            },
            "remarks": null,
            "entry_type": "IMPLEMENTED",
            "bid_date": null,
            "year": 2023,
            "abc_published": null,
            "prop_takers": null,
            "prop_infra": null,
            "time_extension": null
          }
        }), db_connection).await,
convert_projects_into_active_model_v2(&json!({
          "id": "1135",
          "attributes": {
            "name": "Concreting of Road (1/2 lane) from Barangay Market to Cockpit Area, Macasing\n",
            "code": "1SAIL23 - 8000 - A47",
            "location": "[\"44\"]",
            "cost": 386294,
            "contractor": {
              "data": {
                "id": "1",
                "attributes": {
                  "name": "CTG Construction",
                  "street_purok": "Purok 1"
                }
              }
            },
            "duration": null,
            "adjusted": null,
            "total": null,
            "weight": null,
            "prop_sdg": "[]",
            "prop_fund": "19",
            "prop_type": "1",
            "prop_category": "2",
            "prop_sector": "[]",
            "prop_assign": "2",
            "accom_total": 30,
            "accom_value": null,
            "accom_date": null,
            "accom_elapse": null,
            "accom_slippage": null,
            "accom_period": null,
            "accom_todate": null,
            "accom_qty": null,
            "accom_amt": null,
            "date": null,
            "status": "CREATED",
            "start": null,
            "target": null,
            "office": "Engineer's Office",
            "assigned": null,
            "appropriation": 400000,
            "abc": 0,
            "status_id": {
              "data": null
            },
            "remarks": "30% physical accomplishment as of oct. 21, 2023\n\n90% physical accomplishment as of Nov. 3, 2023",
            "entry_type": "IMPLEMENTED",
            "bid_date": null,
            "year": 2023,
            "abc_published": null,
            "prop_takers": null,
            "prop_infra": null,
            "time_extension": null
          }
        }), db_connection).await,
convert_projects_into_active_model_v2(&json!({
          "id": "1136",
          "attributes": {
            "name": "Repair/Reblocking of Road Including Area of Police Outpost, Barangay Macasing\n",
            "code": "1SAIL23 - 8000 - A48",
            "location": "[\"44\"]",
            "cost": 0,
            "contractor": {
              "data": null
            },
            "duration": null,
            "adjusted": null,
            "total": null,
            "weight": null,
            "prop_sdg": "[]",
            "prop_fund": "19",
            "prop_type": "1",
            "prop_category": "3",
            "prop_sector": "[]",
            "prop_assign": "",
            "accom_total": 0,
            "accom_value": null,
            "accom_date": null,
            "accom_elapse": null,
            "accom_slippage": null,
            "accom_period": null,
            "accom_todate": null,
            "accom_qty": null,
            "accom_amt": null,
            "date": null,
            "status": "CREATED",
            "start": null,
            "target": null,
            "office": "Engineer's Office",
            "assigned": null,
            "appropriation": 1000000,
            "abc": 0,
            "status_id": {
              "data": null
            },
            "remarks": null,
            "entry_type": "IMPLEMENTED",
            "bid_date": null,
            "year": 2023,
            "abc_published": null,
            "prop_takers": null,
            "prop_infra": null,
            "time_extension": null
          }
        }), db_connection).await,
convert_projects_into_active_model_v2(&json!({
          "id": "1137",
          "attributes": {
            "name": "Concreting of Road at Purok 1, Barangay Manga",
            "code": "1SAIL23-8000-A49",
            "location": "[\"45\"]",
            "cost": 968588.1,
            "contractor": {
              "data": {
                "id": "19",
                "attributes": {
                  "name": "Jenrich Construction",
                  "street_purok": null
                }
              }
            },
            "duration": 50,
            "adjusted": null,
            "total": null,
            "weight": null,
            "prop_sdg": "[]",
            "prop_fund": "19",
            "prop_type": "1",
            "prop_category": "2",
            "prop_sector": "[]",
            "prop_assign": "2",
            "accom_total": 100,
            "accom_value": null,
            "accom_date": null,
            "accom_elapse": null,
            "accom_slippage": null,
            "accom_period": null,
            "accom_todate": null,
            "accom_qty": null,
            "accom_amt": null,
            "date": null,
            "status": "CREATED",
            "start": "2023-07-17",
            "target": "2023-09-05",
            "office": "Engineer's Office",
            "assigned": null,
            "appropriation": 1000000,
            "abc": 0,
            "status_id": {
              "data": null
            },
            "remarks": "100% physically completed as per report of project incharge\n\nActual date Completed: 8/28/2023",
            "entry_type": "IMPLEMENTED",
            "bid_date": null,
            "year": 2023,
            "abc_published": null,
            "prop_takers": "\"JOEL HOFILENA 09491215525\"",
            "prop_infra": null,
            "time_extension": null
          }
        }), db_connection).await,
convert_projects_into_active_model_v2(&json!({
          "id": "1138",
          "attributes": {
            "name": "Concreting of Road at Purok 4, Barangay Manga\n",
            "code": "1SAIL23-8000-A50",
            "location": "[\"45\"]",
            "cost": 489000.2,
            "contractor": {
              "data": {
                "id": "8",
                "attributes": {
                  "name": "JAMT Construction",
                  "street_purok": null
                }
              }
            },
            "duration": 60,
            "adjusted": null,
            "total": null,
            "weight": null,
            "prop_sdg": "[]",
            "prop_fund": "19",
            "prop_type": "1",
            "prop_category": "2",
            "prop_sector": "[]",
            "prop_assign": "2",
            "accom_total": 100,
            "accom_value": null,
            "accom_date": null,
            "accom_elapse": null,
            "accom_slippage": null,
            "accom_period": null,
            "accom_todate": null,
            "accom_qty": null,
            "accom_amt": null,
            "date": null,
            "status": "CREATED",
            "start": "2023-11-07",
            "target": "2024-01-06",
            "office": "Engineer's Office",
            "assigned": null,
            "appropriation": 500000,
            "abc": 500000,
            "status_id": {
              "data": null
            },
            "remarks": "* 100% physically completed base on approved technical plans & specifications\n* Actual Completion Date: 12/7/2023\n",
            "entry_type": "IMPLEMENTED",
            "bid_date": null,
            "year": 2023,
            "abc_published": null,
            "prop_takers": "CAPT. BUSTAMANTE",
            "prop_infra": null,
            "time_extension": null
          }
        }), db_connection).await,
convert_projects_into_active_model_v2(&json!({
          "id": "1139",
          "attributes": {
            "name": " Concreting of Road at Purok Palmera, Barangay Napolan",
            "code": "1SAIL23-8000-A52",
            "location": "[\"10\"]",
            "cost": 478470,
            "contractor": {
              "data": {
                "id": "19",
                "attributes": {
                  "name": "Jenrich Construction",
                  "street_purok": null
                }
              }
            },
            "duration": 45,
            "adjusted": null,
            "total": null,
            "weight": null,
            "prop_sdg": "[]",
            "prop_fund": "19",
            "prop_type": "1",
            "prop_category": "2",
            "prop_sector": "[]",
            "prop_assign": "2",
            "accom_total": 100,
            "accom_value": null,
            "accom_date": null,
            "accom_elapse": null,
            "accom_slippage": null,
            "accom_period": null,
            "accom_todate": null,
            "accom_qty": null,
            "accom_amt": null,
            "date": null,
            "status": "CREATED",
            "start": "2023-06-19",
            "target": "2023-08-03",
            "office": "Engineer's Office",
            "assigned": null,
            "appropriation": 500000,
            "abc": 0,
            "status_id": {
              "data": null
            },
            "remarks": "\"* 100% physically completed based on approved technical plans & specifications\n* w/ time extension due to VO No.1\n* Adjusted Target Date of Completion: Aug. 18, 2023\"\n",
            "entry_type": "IMPLEMENTED",
            "bid_date": null,
            "year": 2023,
            "abc_published": null,
            "prop_takers": null,
            "prop_infra": null,
            "time_extension": null
          }
        }), db_connection).await,
        convert_projects_into_active_model_v2(&json!({
          "id": "1140",
          "attributes": {
            "name": "Concreting of Road at Purok Azucina, Barangay Napolan\n",
            "code": "1SAIL23-8000-A53",
            "location": "[\"10\"]",
            "cost": 580756.8,
            "contractor": {
              "data": {
                "id": "19",
                "attributes": {
                  "name": "Jenrich Construction",
                  "street_purok": null
                }
              }
            },
            "duration": 45,
            "adjusted": null,
            "total": null,
            "weight": null,
            "prop_sdg": "[]",
            "prop_fund": "19",
            "prop_type": "1",
            "prop_category": "2",
            "prop_sector": "[]",
            "prop_assign": "2",
            "accom_total": 100,
            "accom_value": null,
            "accom_date": null,
            "accom_elapse": null,
            "accom_slippage": null,
            "accom_period": null,
            "accom_todate": null,
            "accom_qty": null,
            "accom_amt": null,
            "date": null,
            "status": "CREATED",
            "start": "2023-07-10",
            "target": "2023-08-24",
            "office": "Engineer's Office",
            "assigned": null,
            "appropriation": 600000,
            "abc": 0,
            "status_id": {
              "data": null
            },
            "remarks": "\"* 100% physically completed base on approved technical plans & specifications\n* w/ time extension as per approved Suspension Order No. 1\n* Adjusted Targe Date of Completion: Dec. 25, 2023\"\n* Actual Completion date: 12/20/2023",
            "entry_type": "IMPLEMENTED",
            "bid_date": null,
            "year": 2023,
            "abc_published": null,
            "prop_takers": "KAGAWAD ALBARACIN",
            "prop_infra": null,
            "time_extension": null
          }
        }), db_connection).await,
convert_projects_into_active_model_v2(&json!({
          "id": "1141",
          "attributes": {
            "name": "Concreting of Road at Purok Centro, Barangay Napolan\n",
            "code": "1SAIL23-8000-A54",
            "location": "[\"10\"]",
            "cost": 481119.65,
            "contractor": {
              "data": {
                "id": "24",
                "attributes": {
                  "name": "MG Construction",
                  "street_purok": null
                }
              }
            },
            "duration": 45,
            "adjusted": null,
            "total": null,
            "weight": null,
            "prop_sdg": "[]",
            "prop_fund": "19",
            "prop_type": "1",
            "prop_category": "2",
            "prop_sector": "[]",
            "prop_assign": "2",
            "accom_total": 100,
            "accom_value": null,
            "accom_date": null,
            "accom_elapse": null,
            "accom_slippage": null,
            "accom_period": null,
            "accom_todate": null,
            "accom_qty": null,
            "accom_amt": null,
            "date": null,
            "status": "CREATED",
            "start": "2023-08-21",
            "target": "2023-10-05",
            "office": "Engineer's Office",
            "assigned": null,
            "appropriation": 500000,
            "abc": 0,
            "status_id": {
              "data": null
            },
            "remarks": "* 100% physically completed base on approved technical plans & specifications\n* w/ time extension as per approved Suspension Order No. 1\n* Adjusted Targe Date of Completion: Dec. 6, 2023\"\n",
            "entry_type": "IMPLEMENTED",
            "bid_date": null,
            "year": 2023,
            "abc_published": null,
            "prop_takers": "CORPO",
            "prop_infra": null,
            "time_extension": null
          }
        }), db_connection).await,
convert_projects_into_active_model_v2(&json!({
          "id": "1142",
          "attributes": {
            "name": "Concreting of Road at Purok Santan, Barangay Napolan\n",
            "code": "1SAIL23-8000-A55",
            "location": "[\"10\"]",
            "cost": 481405.24,
            "contractor": {
              "data": {
                "id": "24",
                "attributes": {
                  "name": "MG Construction",
                  "street_purok": null
                }
              }
            },
            "duration": 45,
            "adjusted": null,
            "total": null,
            "weight": null,
            "prop_sdg": "[]",
            "prop_fund": "19",
            "prop_type": "1",
            "prop_category": "2",
            "prop_sector": "[]",
            "prop_assign": "2",
            "accom_total": 100,
            "accom_value": null,
            "accom_date": null,
            "accom_elapse": null,
            "accom_slippage": null,
            "accom_period": null,
            "accom_todate": null,
            "accom_qty": null,
            "accom_amt": null,
            "date": null,
            "status": "CREATED",
            "start": "2023-09-04",
            "target": "2023-10-19",
            "office": "Engineer's Office",
            "assigned": null,
            "appropriation": 500000,
            "abc": 0,
            "status_id": {
              "data": null
            },
            "remarks": "\"* 100% physically completed base on approved technical plans & specifications\n* w/ time extension as per approved Suspension Order No. 1\n* Adjusted Targe Date of Completion: Nov. 29, 2023\"\n",
            "entry_type": "IMPLEMENTED",
            "bid_date": null,
            "year": 2023,
            "abc_published": null,
            "prop_takers": "CORPO",
            "prop_infra": null,
            "time_extension": null
          }
        }), db_connection).await,
convert_projects_into_active_model_v2(&json!({
          "id": "1143",
          "attributes": {
            "name": " Concreting of Road at Purok Sampaguita, Barangay Napolan\n",
            "code": "1SAIL23-8000-A56",
            "location": "[\"10\"]",
            "cost": 287388.04,
            "contractor": {
              "data": {
                "id": "19",
                "attributes": {
                  "name": "Jenrich Construction",
                  "street_purok": null
                }
              }
            },
            "duration": 45,
            "adjusted": null,
            "total": null,
            "weight": null,
            "prop_sdg": "[]",
            "prop_fund": "19",
            "prop_type": "1",
            "prop_category": "2",
            "prop_sector": "[]",
            "prop_assign": "2",
            "accom_total": 100,
            "accom_value": null,
            "accom_date": null,
            "accom_elapse": null,
            "accom_slippage": null,
            "accom_period": null,
            "accom_todate": null,
            "accom_qty": null,
            "accom_amt": null,
            "date": null,
            "status": "CREATED",
            "start": "2023-07-24",
            "target": "2023-09-07",
            "office": "Engineer's Office",
            "assigned": null,
            "appropriation": 300000,
            "abc": 0,
            "status_id": {
              "data": null
            },
            "remarks": "\"* 100% physically completed base on approved technical plans & specifications\n* w/ time extension as per approved Suspension Order No. 1\n* Adjusted Targe Date of Completion: Nov. 26, 2023\"\n* Actual date Completed: 12/24/2023\n",
            "entry_type": "IMPLEMENTED",
            "bid_date": null,
            "year": 2023,
            "abc_published": null,
            "prop_takers": "KAGAWAD ALBARACIN",
            "prop_infra": null,
            "time_extension": null
          }
        }), db_connection).await,
convert_projects_into_active_model_v2(&json!({
          "id": "1144",
          "attributes": {
            "name": "Concreting of Road at Purok Waling-waling, Barangay Napolan\n",
            "code": "1SAIL23-8000-A57",
            "location": "[\"10\"]",
            "cost": 481256.57,
            "contractor": {
              "data": {
                "id": "19",
                "attributes": {
                  "name": "Jenrich Construction",
                  "street_purok": null
                }
              }
            },
            "duration": 45,
            "adjusted": null,
            "total": null,
            "weight": null,
            "prop_sdg": "[]",
            "prop_fund": "19",
            "prop_type": "1",
            "prop_category": "2",
            "prop_sector": "[]",
            "prop_assign": "2",
            "accom_total": 100,
            "accom_value": null,
            "accom_date": null,
            "accom_elapse": null,
            "accom_slippage": null,
            "accom_period": null,
            "accom_todate": null,
            "accom_qty": null,
            "accom_amt": null,
            "date": null,
            "status": "CREATED",
            "start": "2023-09-11",
            "target": "2023-10-26",
            "office": "Engineer's Office",
            "assigned": null,
            "appropriation": 500000,
            "abc": 0,
            "status_id": {
              "data": null
            },
            "remarks": "\"* 100% physically completed base on approved technical plans & specifications\n* w/ time extension as per approved Suspension Order No. 1\n* Adjusted Targe Date of Completion: Dec.6, 2023\"\n* Actual Date Completed:12/5/2023",
            "entry_type": "IMPLEMENTED",
            "bid_date": null,
            "year": 2023,
            "abc_published": null,
            "prop_takers": "KAGAWAD ALBARACIN",
            "prop_infra": null,
            "time_extension": null
          }
        }), db_connection).await,
convert_projects_into_active_model_v2(&json!({
          "id": "1145",
          "attributes": {
            "name": "Concreting of Road at Purok San Francisco, Barangay Napolan",
            "code": "1SAIL23-8000-A58",
            "location": "[\"10\"]",
            "cost": 241797.08,
            "contractor": {
              "data": {
                "id": "19",
                "attributes": {
                  "name": "Jenrich Construction",
                  "street_purok": null
                }
              }
            },
            "duration": null,
            "adjusted": null,
            "total": null,
            "weight": null,
            "prop_sdg": "[]",
            "prop_fund": "19",
            "prop_type": "1",
            "prop_category": "2",
            "prop_sector": "[]",
            "prop_assign": "2",
            "accom_total": 30,
            "accom_value": null,
            "accom_date": null,
            "accom_elapse": null,
            "accom_slippage": null,
            "accom_period": null,
            "accom_todate": null,
            "accom_qty": null,
            "accom_amt": null,
            "date": null,
            "status": "CREATED",
            "start": null,
            "target": null,
            "office": "Engineer's Office",
            "assigned": null,
            "appropriation": 250000,
            "abc": 0,
            "status_id": {
              "data": null
            },
            "remarks": "30% PHYSICAL ACCOMPLISHMENT AS OF MARCH 8, 2024",
            "entry_type": "IMPLEMENTED",
            "bid_date": null,
            "year": 2023,
            "abc_published": null,
            "prop_takers": "KAGAWAD ALBARACIN",
            "prop_infra": null,
            "time_extension": null
          }
        }), db_connection).await,
convert_projects_into_active_model_v2(&json!({
          "id": "1146",
          "attributes": {
            "name": "Concreting of Road Leading to Palpalan Peak, Barangay Palpalan\n",
            "code": "1SAIL23-8000-A59",
            "location": "[\"47\"]",
            "cost": 0,
            "contractor": {
              "data": null
            },
            "duration": null,
            "adjusted": null,
            "total": null,
            "weight": null,
            "prop_sdg": "[]",
            "prop_fund": "5",
            "prop_type": null,
            "prop_category": null,
            "prop_sector": "[]",
            "prop_assign": "2",
            "accom_total": 0,
            "accom_value": null,
            "accom_date": null,
            "accom_elapse": null,
            "accom_slippage": null,
            "accom_period": null,
            "accom_todate": null,
            "accom_qty": null,
            "accom_amt": null,
            "date": null,
            "status": "CREATED",
            "start": null,
            "target": null,
            "office": "Engineer's Office",
            "assigned": null,
            "appropriation": 600000,
            "abc": 0,
            "status_id": {
              "data": null
            },
            "remarks": "BIDDED but NYS\n",
            "entry_type": "IMPLEMENTED",
            "bid_date": null,
            "year": 2023,
            "abc_published": null,
            "prop_takers": "CAPT. DENOPOL",
            "prop_infra": null,
            "time_extension": null
          }
        }), db_connection).await,
convert_projects_into_active_model_v2(&json!({
          "id": "1147",
          "attributes": {
            "name": "Concreting of Road at Purok Tinangkong, Barangay Palpalan\n",
            "code": "1SAIL23 - 8000 - A60",
            "location": "[\"47\"]",
            "cost": 0,
            "contractor": {
              "data": null
            },
            "duration": null,
            "adjusted": null,
            "total": null,
            "weight": null,
            "prop_sdg": "[]",
            "prop_fund": "19",
            "prop_type": "1",
            "prop_category": "2",
            "prop_sector": "[]",
            "prop_assign": "2",
            "accom_total": 0,
            "accom_value": null,
            "accom_date": null,
            "accom_elapse": null,
            "accom_slippage": null,
            "accom_period": null,
            "accom_todate": null,
            "accom_qty": null,
            "accom_amt": null,
            "date": null,
            "status": "CREATED",
            "start": null,
            "target": null,
            "office": "Engineer's Office",
            "assigned": null,
            "appropriation": 800000,
            "abc": 0,
            "status_id": {
              "data": null
            },
            "remarks": null,
            "entry_type": "IMPLEMENTED",
            "bid_date": null,
            "year": 2023,
            "abc_published": null,
            "prop_takers": null,
            "prop_infra": null,
            "time_extension": null
          }
        }), db_connection).await,
convert_projects_into_active_model_v2(&json!({
          "id": "1148",
          "attributes": {
            "name": "Gravelling of Road at Purok Gemelina, Barangay Pedilunan\n",
            "code": "1SAIL23 - 8000 - A61",
            "location": "[\"48\"]",
            "cost": 482968,
            "contractor": {
              "data": {
                "id": "1",
                "attributes": {
                  "name": "CTG Construction",
                  "street_purok": "Purok 1"
                }
              }
            },
            "duration": null,
            "adjusted": null,
            "total": null,
            "weight": null,
            "prop_sdg": "[]",
            "prop_fund": "19",
            "prop_type": "1",
            "prop_category": "4",
            "prop_sector": "[]",
            "prop_assign": "2",
            "accom_total": 90,
            "accom_value": null,
            "accom_date": null,
            "accom_elapse": null,
            "accom_slippage": null,
            "accom_period": null,
            "accom_todate": null,
            "accom_qty": null,
            "accom_amt": null,
            "date": null,
            "status": "CREATED",
            "start": null,
            "target": null,
            "office": "Engineer's Office",
            "assigned": null,
            "appropriation": 500000,
            "abc": 0,
            "status_id": {
              "data": null
            },
            "remarks": "On-going with a physical accomplishment of 90% as of Oct. 6, 2023 as per report of project incharge\n\n95% physical accomplishment as of Nov. 3, 2023",
            "entry_type": "IMPLEMENTED",
            "bid_date": null,
            "year": 2023,
            "abc_published": null,
            "prop_takers": null,
            "prop_infra": null,
            "time_extension": null
          }
        }), db_connection).await,
convert_projects_into_active_model_v2(&json!({
          "id": "1149",
          "attributes": {
            "name": "Concreting of Road (1/2 Lane) Leading to Mabait, Barangay Poloyagan\n",
            "code": "1SAIL23-8000-A62",
            "location": "[\"49\"]",
            "cost": 1732531.3,
            "contractor": {
              "data": {
                "id": "1",
                "attributes": {
                  "name": "CTG Construction",
                  "street_purok": "Purok 1"
                }
              }
            },
            "duration": 90,
            "adjusted": null,
            "total": null,
            "weight": null,
            "prop_sdg": "[]",
            "prop_fund": "19",
            "prop_type": "1",
            "prop_category": "2",
            "prop_sector": "[]",
            "prop_assign": "2",
            "accom_total": 100,
            "accom_value": null,
            "accom_date": null,
            "accom_elapse": null,
            "accom_slippage": null,
            "accom_period": null,
            "accom_todate": null,
            "accom_qty": null,
            "accom_amt": null,
            "date": null,
            "status": "CREATED",
            "start": "2023-07-03",
            "target": "2023-10-01",
            "office": "Engineer's Office",
            "assigned": null,
            "appropriation": 1800000,
            "abc": 0,
            "status_id": {
              "data": null
            },
            "remarks": "\"* 100% physically completed base on approved technical plans & specifications\n* w/ time extension as per approved Suspension Order No. 1\n* Adjusted Targe Date of Completion: Dec. 3, 2023\"\n* Actual Date Completed:12/1/2023",
            "entry_type": "IMPLEMENTED",
            "bid_date": null,
            "year": 2023,
            "abc_published": null,
            "prop_takers": "\"CAPT. POLOYAGAN EDGAR REVELO\"",
            "prop_infra": null,
            "time_extension": null
          }
        }), db_connection).await,
        convert_projects_into_active_model_v2(&json!({
          "id": "1150",
          "attributes": {
            "name": "Concreting of Road Leading to Elem School Site, Barangay Poloyagan\n",
            "code": "1SAIL23-8000-A63",
            "location": "[\"49\"]",
            "cost": 170972,
            "contractor": {
              "data": {
                "id": "1",
                "attributes": {
                  "name": "CTG Construction",
                  "street_purok": "Purok 1"
                }
              }
            },
            "duration": 45,
            "adjusted": null,
            "total": null,
            "weight": null,
            "prop_sdg": "[]",
            "prop_fund": "19",
            "prop_type": "1",
            "prop_category": "1",
            "prop_sector": "[]",
            "prop_assign": "2",
            "accom_total": 100,
            "accom_value": null,
            "accom_date": null,
            "accom_elapse": null,
            "accom_slippage": null,
            "accom_period": null,
            "accom_todate": null,
            "accom_qty": null,
            "accom_amt": null,
            "date": null,
            "status": "CREATED",
            "start": "2023-07-24",
            "target": "2023-09-07",
            "office": "Engineer's Office",
            "assigned": null,
            "appropriation": 180000,
            "abc": 0,
            "status_id": {
              "data": null
            },
            "remarks": "100% physically completed as per report of project incharge as of July 28, 2023",
            "entry_type": "IMPLEMENTED",
            "bid_date": null,
            "year": 2023,
            "abc_published": null,
            "prop_takers": "KAG. EDGAR REVELO",
            "prop_infra": null,
            "time_extension": null
          }
        }), db_connection).await,

convert_projects_into_active_model_v2(&json!({
          "id": "1151",
          "attributes": {
            "name": "Improvement of Alley (Jct Sabado St - Alano St) Purok Rubia, Barangay San Francisco\n",
            "code": "1SAIL23 - 8000 - A64",
            "location": "[]",
            "cost": 0,
            "contractor": {
              "data": null
            },
            "duration": null,
            "adjusted": null,
            "total": null,
            "weight": null,
            "prop_sdg": "[]",
            "prop_fund": "19",
            "prop_type": "1",
            "prop_category": "4",
            "prop_sector": "[]",
            "prop_assign": "2",
            "accom_total": 0,
            "accom_value": null,
            "accom_date": null,
            "accom_elapse": null,
            "accom_slippage": null,
            "accom_period": null,
            "accom_todate": null,
            "accom_qty": null,
            "accom_amt": null,
            "date": null,
            "status": "CREATED",
            "start": null,
            "target": null,
            "office": "Engineer's Office",
            "assigned": null,
            "appropriation": 200000,
            "abc": 0,
            "status_id": {
              "data": null
            },
            "remarks": null,
            "entry_type": "IMPLEMENTED",
            "bid_date": null,
            "year": 2023,
            "abc_published": null,
            "prop_takers": null,
            "prop_infra": null,
            "time_extension": null
          }
        }), db_connection).await,

convert_projects_into_active_model_v2(&json!({
          "id": "1152",
          "attributes": {
            "name": "Improvement of Road at Prk Esmeralda B, Barangay San Jose\n",
            "code": "1SAIL23 - 8000 - A65",
            "location": "[]",
            "cost": 0,
            "contractor": {
              "data": null
            },
            "duration": null,
            "adjusted": null,
            "total": null,
            "weight": null,
            "prop_sdg": "[]",
            "prop_fund": "19",
            "prop_type": "1",
            "prop_category": "4",
            "prop_sector": "[]",
            "prop_assign": "2",
            "accom_total": 0,
            "accom_value": null,
            "accom_date": null,
            "accom_elapse": null,
            "accom_slippage": null,
            "accom_period": null,
            "accom_todate": null,
            "accom_qty": null,
            "accom_amt": null,
            "date": null,
            "status": "CREATED",
            "start": null,
            "target": null,
            "office": "Engineer's Office",
            "assigned": null,
            "appropriation": 1000000,
            "abc": 0,
            "status_id": {
              "data": null
            },
            "remarks": null,
            "entry_type": "IMPLEMENTED",
            "bid_date": null,
            "year": 2023,
            "abc_published": null,
            "prop_takers": null,
            "prop_infra": null,
            "time_extension": null
          }
        }), db_connection).await,

convert_projects_into_active_model_v2(&json!({
          "id": "1153",
          "attributes": {
            "name": "Concreting of Alley at Purok Santan B & A, Barangay San Jose\n",
            "code": "1SAIL23 - 8000 - A66",
            "location": "[\"12\"]",
            "cost": 0,
            "contractor": {
              "data": null
            },
            "duration": null,
            "adjusted": null,
            "total": null,
            "weight": null,
            "prop_sdg": "[]",
            "prop_fund": "19",
            "prop_type": "1",
            "prop_category": "2",
            "prop_sector": "[]",
            "prop_assign": "2",
            "accom_total": 95,
            "accom_value": null,
            "accom_date": null,
            "accom_elapse": null,
            "accom_slippage": null,
            "accom_period": null,
            "accom_todate": null,
            "accom_qty": null,
            "accom_amt": null,
            "date": null,
            "status": "CREATED",
            "start": null,
            "target": null,
            "office": "Engineer's Office",
            "assigned": null,
            "appropriation": 500000,
            "abc": 0,
            "status_id": {
              "data": null
            },
            "remarks": "95% physical accomplishment as of Oct. 21, 2023\n\n100% PHYSICALLY COMPLETED AS OF NOV. 10, 2023",
            "entry_type": "IMPLEMENTED",
            "bid_date": null,
            "year": 2023,
            "abc_published": null,
            "prop_takers": null,
            "prop_infra": null,
            "time_extension": null
          }
        }), db_connection).await,

convert_projects_into_active_model_v2(&json!({
          "id": "1154",
          "attributes": {
            "name": "Concreting of Road at Salera Street (Phase II) Barangay Sta, Lucia\n",
            "code": "1SAIL23-8000-A67",
            "location": "[\"14\"]",
            "cost": 967965.06,
            "contractor": {
              "data": {
                "id": "8",
                "attributes": {
                  "name": "JAMT Construction",
                  "street_purok": null
                }
              }
            },
            "duration": 100,
            "adjusted": null,
            "total": null,
            "weight": null,
            "prop_sdg": "[]",
            "prop_fund": "19",
            "prop_type": "1",
            "prop_category": "2",
            "prop_sector": "[]",
            "prop_assign": "2",
            "accom_total": 100,
            "accom_value": null,
            "accom_date": null,
            "accom_elapse": null,
            "accom_slippage": null,
            "accom_period": null,
            "accom_todate": null,
            "accom_qty": null,
            "accom_amt": null,
            "date": null,
            "status": "CREATED",
            "start": "2023-07-24",
            "target": "2023-11-01",
            "office": "Engineer's Office",
            "assigned": null,
            "appropriation": 1000000,
            "abc": 0,
            "status_id": {
              "data": null
            },
            "remarks": "100% physically completed \n\nActual Date Completed: 10/5/2023",
            "entry_type": "IMPLEMENTED",
            "bid_date": null,
            "year": 2023,
            "abc_published": null,
            "prop_takers": "ARIES MADARANG",
            "prop_infra": null,
            "time_extension": null
          }
        }), db_connection).await,

convert_projects_into_active_model_v2(&json!({
          "id": "1155",
          "attributes": {
            "name": "Concreting of Alley at Purok Makiangayon beside Buffalo (Phase II) Barangay Sta, Lucia\n",
            "code": "1SAIL23 - 8000 - A68",
            "location": "[\"14\"]",
            "cost": 0,
            "contractor": {
              "data": null
            },
            "duration": null,
            "adjusted": null,
            "total": null,
            "weight": null,
            "prop_sdg": "[]",
            "prop_fund": "19",
            "prop_type": "1",
            "prop_category": "2",
            "prop_sector": "[]",
            "prop_assign": "2",
            "accom_total": 0,
            "accom_value": null,
            "accom_date": null,
            "accom_elapse": null,
            "accom_slippage": null,
            "accom_period": null,
            "accom_todate": null,
            "accom_qty": null,
            "accom_amt": null,
            "date": null,
            "status": "CREATED",
            "start": null,
            "target": null,
            "office": "Engineer's Office",
            "assigned": null,
            "appropriation": 500000,
            "abc": 0,
            "status_id": {
              "data": null
            },
            "remarks": null,
            "entry_type": "IMPLEMENTED",
            "bid_date": null,
            "year": 2023,
            "abc_published": null,
            "prop_takers": null,
            "prop_infra": null,
            "time_extension": null
          }
        }), db_connection).await,

convert_projects_into_active_model_v2(&json!({
          "id": "1156",
          "attributes": {
            "name": "Opening of Alley at Purok Paglaum Blumentritt (Beside Sumalinog Residence), Embankment, Barangay Sta, Lucia\n",
            "code": "1SAIL23 - 8000 - A69",
            "location": "[\"14\"]",
            "cost": 0,
            "contractor": {
              "data": null
            },
            "duration": null,
            "adjusted": null,
            "total": null,
            "weight": null,
            "prop_sdg": "[]",
            "prop_fund": "19",
            "prop_type": "1",
            "prop_category": "2",
            "prop_sector": "[]",
            "prop_assign": "2",
            "accom_total": 0,
            "accom_value": null,
            "accom_date": null,
            "accom_elapse": null,
            "accom_slippage": null,
            "accom_period": null,
            "accom_todate": null,
            "accom_qty": null,
            "accom_amt": null,
            "date": null,
            "status": "CREATED",
            "start": null,
            "target": null,
            "office": "Engineer's Office",
            "assigned": null,
            "appropriation": 1000000,
            "abc": 0,
            "status_id": {
              "data": null
            },
            "remarks": null,
            "entry_type": "IMPLEMENTED",
            "bid_date": null,
            "year": 2023,
            "abc_published": null,
            "prop_takers": null,
            "prop_infra": null,
            "time_extension": null
          }
        }), db_connection).await,

convert_projects_into_active_model_v2(&json!({
          "id": "1157",
          "attributes": {
            "name": "Concreting of Jamisola Street, (Jct Aquino St - Duterte ST) Barangay Sta, Lucia\n",
            "code": "1SAIL23 - 8000 - A70",
            "location": "[\"14\"]",
            "cost": 1177110.9,
            "contractor": {
              "data": {
                "id": "5",
                "attributes": {
                  "name": "MOLROW CONSTRUCTION",
                  "street_purok": null
                }
              }
            },
            "duration": 90,
            "adjusted": null,
            "total": null,
            "weight": null,
            "prop_sdg": "[]",
            "prop_fund": "19",
            "prop_type": "1",
            "prop_category": "2",
            "prop_sector": "[]",
            "prop_assign": "2",
            "accom_total": 100,
            "accom_value": null,
            "accom_date": null,
            "accom_elapse": null,
            "accom_slippage": null,
            "accom_period": null,
            "accom_todate": null,
            "accom_qty": null,
            "accom_amt": null,
            "date": null,
            "status": "CREATED",
            "start": "2023-08-14",
            "target": "2023-11-12",
            "office": "Engineer's Office",
            "assigned": null,
            "appropriation": 1240000,
            "abc": 1184336.86,
            "status_id": {
              "data": null
            },
            "remarks": null,
            "entry_type": "IMPLEMENTED",
            "bid_date": null,
            "year": 2023,
            "abc_published": null,
            "prop_takers": null,
            "prop_infra": null,
            "time_extension": null
          }
        }), db_connection).await,

convert_projects_into_active_model_v2(&json!({
          "id": "1158",
          "attributes": {
            "name": "Concreting of Road at Purok Orchids (Leading to Alano St) Barangay Sta. Maria",
            "code": "1SAIL23 - 8000 - A71",
            "location": "[\"15\"]",
            "cost": 0,
            "contractor": {
              "data": null
            },
            "duration": null,
            "adjusted": null,
            "total": null,
            "weight": null,
            "prop_sdg": "[]",
            "prop_fund": "19",
            "prop_type": "1",
            "prop_category": "2",
            "prop_sector": "[]",
            "prop_assign": "2",
            "accom_total": 100,
            "accom_value": null,
            "accom_date": null,
            "accom_elapse": null,
            "accom_slippage": null,
            "accom_period": null,
            "accom_todate": null,
            "accom_qty": null,
            "accom_amt": null,
            "date": null,
            "status": "CREATED",
            "start": null,
            "target": null,
            "office": "Engineer's Office",
            "assigned": null,
            "appropriation": 500000,
            "abc": 0,
            "status_id": {
              "data": null
            },
            "remarks": "100% physically completed as per report of project incharge as of Sept. 1, 2023 reporting period",
            "entry_type": "IMPLEMENTED",
            "bid_date": null,
            "year": 2023,
            "abc_published": null,
            "prop_takers": null,
            "prop_infra": null,
            "time_extension": null
          }
        }), db_connection).await,

convert_projects_into_active_model_v2(&json!({
          "id": "1159",
          "attributes": {
            "name": "Improvement of Road with Box Culvert at Lapu-Lapu St, Barangay Sta. Maria\n",
            "code": "1SAIL23 - 8000 - A72",
            "location": "[\"15\"]",
            "cost": 0,
            "contractor": {
              "data": null
            },
            "duration": null,
            "adjusted": null,
            "total": null,
            "weight": null,
            "prop_sdg": "[]",
            "prop_fund": "19",
            "prop_type": "1",
            "prop_category": "4",
            "prop_sector": "[]",
            "prop_assign": "2",
            "accom_total": 0,
            "accom_value": null,
            "accom_date": null,
            "accom_elapse": null,
            "accom_slippage": null,
            "accom_period": null,
            "accom_todate": null,
            "accom_qty": null,
            "accom_amt": null,
            "date": null,
            "status": "CREATED",
            "start": null,
            "target": null,
            "office": "Engineer's Office",
            "assigned": null,
            "appropriation": 500000,
            "abc": 0,
            "status_id": {
              "data": null
            },
            "remarks": null,
            "entry_type": "IMPLEMENTED",
            "bid_date": null,
            "year": 2023,
            "abc_published": null,
            "prop_takers": null,
            "prop_infra": null,
            "time_extension": null
          }
        }), db_connection).await,
        convert_projects_into_active_model_v2(&json!({
          "id": "1160",
          "attributes": {
            "name": "Improvement of Road at Purok Antorium, Barangay Tawagan Sur\n",
            "code": "1SAIL23 - 8000 - A73",
            "location": "[\"50\"]",
            "cost": 0,
            "contractor": {
              "data": null
            },
            "duration": null,
            "adjusted": null,
            "total": null,
            "weight": null,
            "prop_sdg": "[]",
            "prop_fund": "19",
            "prop_type": "1",
            "prop_category": "4",
            "prop_sector": "[]",
            "prop_assign": "2",
            "accom_total": 0,
            "accom_value": null,
            "accom_date": null,
            "accom_elapse": null,
            "accom_slippage": null,
            "accom_period": null,
            "accom_todate": null,
            "accom_qty": null,
            "accom_amt": null,
            "date": null,
            "status": "CREATED",
            "start": null,
            "target": null,
            "office": "Engineer's Office",
            "assigned": null,
            "appropriation": 700000,
            "abc": 0,
            "status_id": {
              "data": null
            },
            "remarks": null,
            "entry_type": "IMPLEMENTED",
            "bid_date": null,
            "year": 2023,
            "abc_published": null,
            "prop_takers": null,
            "prop_infra": null,
            "time_extension": null
          }
        }), db_connection).await,
convert_projects_into_active_model_v2(&json!({
          "id": "1161",
          "attributes": {
            "name": "Concreting of Road at Purok Caimito, Barangay Tuburan\n",
            "code": "1SAIL23 - 8000 - A74",
            "location": "[\"19\"]",
            "cost": 0,
            "contractor": {
              "data": null
            },
            "duration": null,
            "adjusted": null,
            "total": null,
            "weight": null,
            "prop_sdg": "[]",
            "prop_fund": "19",
            "prop_type": "1",
            "prop_category": "2",
            "prop_sector": "[]",
            "prop_assign": "2",
            "accom_total": 100,
            "accom_value": null,
            "accom_date": null,
            "accom_elapse": null,
            "accom_slippage": null,
            "accom_period": null,
            "accom_todate": null,
            "accom_qty": null,
            "accom_amt": null,
            "date": null,
            "status": "CREATED",
            "start": null,
            "target": null,
            "office": "Engineer's Office",
            "assigned": null,
            "appropriation": 450000,
            "abc": 0,
            "status_id": {
              "data": null
            },
            "remarks": null,
            "entry_type": "IMPLEMENTED",
            "bid_date": null,
            "year": 2023,
            "abc_published": null,
            "prop_takers": null,
            "prop_infra": null,
            "time_extension": null
          }
        }), db_connection).await,
convert_projects_into_active_model_v2(&json!({
          "id": "1162",
          "attributes": {
            "name": "Concreting of Road at Purok Cacao, Barangay Tuburan\n",
            "code": "1SAIL23 - 8000 - A75",
            "location": "[\"19\"]",
            "cost": 0,
            "contractor": {
              "data": null
            },
            "duration": null,
            "adjusted": null,
            "total": null,
            "weight": null,
            "prop_sdg": "[]",
            "prop_fund": "19",
            "prop_type": "1",
            "prop_category": "2",
            "prop_sector": "[]",
            "prop_assign": "2",
            "accom_total": 100,
            "accom_value": null,
            "accom_date": null,
            "accom_elapse": null,
            "accom_slippage": null,
            "accom_period": null,
            "accom_todate": null,
            "accom_qty": null,
            "accom_amt": null,
            "date": null,
            "status": "CREATED",
            "start": null,
            "target": null,
            "office": "Engineer's Office",
            "assigned": null,
            "appropriation": 500000,
            "abc": 0,
            "status_id": {
              "data": null
            },
            "remarks": null,
            "entry_type": "IMPLEMENTED",
            "bid_date": null,
            "year": 2023,
            "abc_published": null,
            "prop_takers": null,
            "prop_infra": null,
            "time_extension": null
          }
        }), db_connection).await,
convert_projects_into_active_model_v2(&json!({
          "id": "1163",
          "attributes": {
            "name": "Reblocking/Rehabilitation of Road at Barangay Tulangan\n",
            "code": "1SAIL23 - 8000 - A76",
            "location": "[\"51\"]",
            "cost": 1936376.1,
            "contractor": {
              "data": {
                "id": "1",
                "attributes": {
                  "name": "CTG Construction",
                  "street_purok": "Purok 1"
                }
              }
            },
            "duration": 90,
            "adjusted": null,
            "total": null,
            "weight": null,
            "prop_sdg": "[]",
            "prop_fund": "17",
            "prop_type": "1",
            "prop_category": "2",
            "prop_sector": "[]",
            "prop_assign": "2",
            "accom_total": 100,
            "accom_value": null,
            "accom_date": null,
            "accom_elapse": null,
            "accom_slippage": null,
            "accom_period": null,
            "accom_todate": null,
            "accom_qty": null,
            "accom_amt": null,
            "date": null,
            "status": "CREATED",
            "start": "2023-09-18",
            "target": "2023-12-17",
            "office": "Engineer's Office",
            "assigned": null,
            "appropriation": 2000000,
            "abc": 0,
            "status_id": {
              "data": null
            },
            "remarks": "* 100% physically completed base on approved technical plans & specifications\n\n* w/ time extension as per approved Suspension Order No. 1\n\n* Adjusted Targe Date of Completion: March 3,2024\"\n",
            "entry_type": "IMPLEMENTED",
            "bid_date": null,
            "year": 2023,
            "abc_published": null,
            "prop_takers": null,
            "prop_infra": null,
            "time_extension": null
          }
        }), db_connection).await,
convert_projects_into_active_model_v2(&json!({
          "id": "1164",
          "attributes": {
            "name": "Concreting of Road Leading to Cemetery, Barangay Tulangan\n",
            "code": "1SAIL23 - 8000 - A77",
            "location": "[\"51\"]",
            "cost": 483479.6,
            "contractor": {
              "data": {
                "id": "1",
                "attributes": {
                  "name": "CTG Construction",
                  "street_purok": "Purok 1"
                }
              }
            },
            "duration": 45,
            "adjusted": null,
            "total": null,
            "weight": null,
            "prop_sdg": "[]",
            "prop_fund": "19",
            "prop_type": "1",
            "prop_category": "2",
            "prop_sector": "[]",
            "prop_assign": "2",
            "accom_total": 70.1,
            "accom_value": null,
            "accom_date": null,
            "accom_elapse": null,
            "accom_slippage": null,
            "accom_period": null,
            "accom_todate": null,
            "accom_qty": null,
            "accom_amt": null,
            "date": null,
            "status": "CREATED",
            "start": "2023-08-21",
            "target": "2023-10-05",
            "office": "Engineer's Office",
            "assigned": null,
            "appropriation": 500000,
            "abc": 0,
            "status_id": {
              "data": null
            },
            "remarks": "\"* On-going implementation of the project w/ physical accomplishment of 70.10% as of  Dec. 4, 2023\n* w/ time extension as per approved Suspension Order No. 1\n* Adjusted Target Date of Completion : December 21, 2023\"\n",
            "entry_type": "IMPLEMENTED",
            "bid_date": null,
            "year": 2023,
            "abc_published": null,
            "prop_takers": "CTG-HERME GUINEA",
            "prop_infra": null,
            "time_extension": null
          }
        }), db_connection).await,
convert_projects_into_active_model_v2(&json!({
          "id": "1165",
          "attributes": {
            "name": "Widening/Rehabilitation of Road at Barangay Tulawas\n",
            "code": "1SAIL23-8000-A78",
            "location": "[\"52\"]",
            "cost": 961204.09,
            "contractor": {
              "data": {
                "id": "19",
                "attributes": {
                  "name": "Jenrich Construction",
                  "street_purok": null
                }
              }
            },
            "duration": null,
            "adjusted": null,
            "total": null,
            "weight": null,
            "prop_sdg": "[]",
            "prop_fund": "19",
            "prop_type": "1",
            "prop_category": "4",
            "prop_sector": "[]",
            "prop_assign": "2",
            "accom_total": 10,
            "accom_value": null,
            "accom_date": null,
            "accom_elapse": null,
            "accom_slippage": null,
            "accom_period": null,
            "accom_todate": null,
            "accom_qty": null,
            "accom_amt": null,
            "date": null,
            "status": "CREATED",
            "start": "",
            "target": null,
            "office": "Engineer's Office",
            "assigned": null,
            "appropriation": 1000000,
            "abc": 961204.09,
            "status_id": {
              "data": null
            },
            "remarks": "On-going implementation of the project\n",
            "entry_type": "IMPLEMENTED",
            "bid_date": null,
            "year": 2023,
            "abc_published": null,
            "prop_takers": null,
            "prop_infra": null,
            "time_extension": null
          }
        }), db_connection).await,
convert_projects_into_active_model_v2(&json!({
          "id": "1166",
          "attributes": {
            "name": "Concreting of Road Leading to Cemetery, Barangay Tulawas",
            "code": "1SAIL23-8000-A79",
            "location": "[\"52\"]",
            "cost": 485025.73,
            "contractor": {
              "data": {
                "id": "19",
                "attributes": {
                  "name": "Jenrich Construction",
                  "street_purok": null
                }
              }
            },
            "duration": 45,
            "adjusted": null,
            "total": null,
            "weight": null,
            "prop_sdg": "[]",
            "prop_fund": "19",
            "prop_type": "1",
            "prop_category": "2",
            "prop_sector": "[]",
            "prop_assign": "2",
            "accom_total": 100,
            "accom_value": null,
            "accom_date": null,
            "accom_elapse": null,
            "accom_slippage": null,
            "accom_period": null,
            "accom_todate": null,
            "accom_qty": null,
            "accom_amt": null,
            "date": null,
            "status": "CREATED",
            "start": "2022-08-07",
            "target": "2022-09-21",
            "office": "Engineer's Office",
            "assigned": null,
            "appropriation": 500000,
            "abc": 0,
            "status_id": {
              "data": null
            },
            "remarks": "\"* 100% physically completed base on approved technical plans & specifications\n* w/ time extension as per approved Suspension Order No. 1\n* Adjusted Targe Date of Completion: Dec. 22, 2023\"\n* Actual Date Completed: 12/20/2023",
            "entry_type": "IMPLEMENTED",
            "bid_date": null,
            "year": 2023,
            "abc_published": null,
            "prop_takers": "CAPT. BABANTA",
            "prop_infra": null,
            "time_extension": null
          }
        }), db_connection).await,
convert_projects_into_active_model_v2(&json!({
          "id": "1167",
          "attributes": {
            "name": "Gravelling of Road at Prk Tampos to Purok Tilid, Barangay Upper Sibatang\n",
            "code": "1SAIL23 - 8000 - A80",
            "location": "[\"53\"]",
            "cost": 389973.77,
            "contractor": {
              "data": {
                "id": "1",
                "attributes": {
                  "name": "CTG Construction",
                  "street_purok": "Purok 1"
                }
              }
            },
            "duration": null,
            "adjusted": null,
            "total": null,
            "weight": null,
            "prop_sdg": "[]",
            "prop_fund": "19",
            "prop_type": "1",
            "prop_category": "4",
            "prop_sector": "[]",
            "prop_assign": "2",
            "accom_total": 80,
            "accom_value": null,
            "accom_date": null,
            "accom_elapse": null,
            "accom_slippage": null,
            "accom_period": null,
            "accom_todate": null,
            "accom_qty": null,
            "accom_amt": null,
            "date": null,
            "status": "CREATED",
            "start": null,
            "target": null,
            "office": "Engineer's Office",
            "assigned": null,
            "appropriation": 400000,
            "abc": 0,
            "status_id": {
              "data": null
            },
            "remarks": "80% physical accomplishment as of Oct. 21, 2023",
            "entry_type": "IMPLEMENTED",
            "bid_date": null,
            "year": 2023,
            "abc_published": null,
            "prop_takers": null,
            "prop_infra": null,
            "time_extension": null
          }
        }), db_connection).await,
convert_projects_into_active_model_v2(&json!({
          "id": "1168",
          "attributes": {
            "name": " Widening of Road at Purok Upper Sibatang to Ditoray, Barangay Upper Sibatang\n",
            "code": "1SAIL23 - 8000 - A81",
            "location": "[\"53\"]",
            "cost": 962176.8,
            "contractor": {
              "data": {
                "id": "19",
                "attributes": {
                  "name": "Jenrich Construction",
                  "street_purok": null
                }
              }
            },
            "duration": null,
            "adjusted": null,
            "total": null,
            "weight": null,
            "prop_sdg": "[]",
            "prop_fund": "19",
            "prop_type": "1",
            "prop_category": "4",
            "prop_sector": "[]",
            "prop_assign": "2",
            "accom_total": 0,
            "accom_value": null,
            "accom_date": null,
            "accom_elapse": null,
            "accom_slippage": null,
            "accom_period": null,
            "accom_todate": null,
            "accom_qty": null,
            "accom_amt": null,
            "date": null,
            "status": "CREATED",
            "start": null,
            "target": null,
            "office": "Engineer's Office",
            "assigned": null,
            "appropriation": 1000000,
            "abc": 0,
            "status_id": {
              "data": null
            },
            "remarks": null,
            "entry_type": "IMPLEMENTED",
            "bid_date": null,
            "year": 2023,
            "abc_published": null,
            "prop_takers": null,
            "prop_infra": null,
            "time_extension": null
          }
        }), db_connection).await,
convert_projects_into_active_model_v2(&json!({
          "id": "1169",
          "attributes": {
            "name": "Construction of Road (Phase II) From Purok Tangigue to School Site (Including Demolition of CR1 & 2 Houses) White Beach\n",
            "code": "1SAIL23 - 8000 - A82",
            "location": "[\"54\"]",
            "cost": 0,
            "contractor": {
              "data": null
            },
            "duration": null,
            "adjusted": null,
            "total": null,
            "weight": null,
            "prop_sdg": "[]",
            "prop_fund": "19",
            "prop_type": "1",
            "prop_category": "1",
            "prop_sector": "[]",
            "prop_assign": "2",
            "accom_total": 0,
            "accom_value": null,
            "accom_date": null,
            "accom_elapse": null,
            "accom_slippage": null,
            "accom_period": null,
            "accom_todate": null,
            "accom_qty": null,
            "accom_amt": null,
            "date": null,
            "status": "CREATED",
            "start": null,
            "target": null,
            "office": "Engineer's Office",
            "assigned": null,
            "appropriation": 800000,
            "abc": 0,
            "status_id": {
              "data": null
            },
            "remarks": null,
            "entry_type": "IMPLEMENTED",
            "bid_date": null,
            "year": 2023,
            "abc_published": null,
            "prop_takers": null,
            "prop_infra": null,
            "time_extension": null
          }
        }), db_connection).await,
        convert_projects_into_active_model_v2(&json!({
          "id": "1170",
          "attributes": {
            "name": "Concreting Of Road Leading to E/S, Barangay Palpalan",
            "code": "1SAIL23-8000-A83",
            "location": "[\"47\"]",
            "cost": 288335.68,
            "contractor": {
              "data": {
                "id": "19",
                "attributes": {
                  "name": "Jenrich Construction",
                  "street_purok": null
                }
              }
            },
            "duration": 45,
            "adjusted": null,
            "total": null,
            "weight": null,
            "prop_sdg": "[]",
            "prop_fund": "19",
            "prop_type": "1",
            "prop_category": "2",
            "prop_sector": "[]",
            "prop_assign": "2",
            "accom_total": 100,
            "accom_value": null,
            "accom_date": null,
            "accom_elapse": null,
            "accom_slippage": null,
            "accom_period": null,
            "accom_todate": null,
            "accom_qty": null,
            "accom_amt": null,
            "date": null,
            "status": "CREATED",
            "start": "2023-08-14",
            "target": "2023-09-28",
            "office": "Engineer's Office",
            "assigned": null,
            "appropriation": 300000,
            "abc": 0,
            "status_id": {
              "data": null
            },
            "remarks": "\"* 100% physically completed base on approved technical plans & specifications\n* w/ time extension as per approved Suspension Order No. 1\n* Adjusted Targe Date of Completion: Dec. 29, 2023\"\n* Actual Date Completed: 12/20/23",
            "entry_type": "IMPLEMENTED",
            "bid_date": null,
            "year": 2023,
            "abc_published": null,
            "prop_takers": "CAPT. DENOPOL",
            "prop_infra": null,
            "time_extension": null
          }
        }), db_connection).await,
convert_projects_into_active_model_v2(&json!({
          "id": "1171",
          "attributes": {
            "name": "Construction of Health Center Phase II, Barangay Alegria\n",
            "code": "1SAIL23-8000-B1",
            "location": "[\"20\"]",
            "cost": 0,
            "contractor": {
              "data": null
            },
            "duration": null,
            "adjusted": null,
            "total": null,
            "weight": null,
            "prop_sdg": "[]",
            "prop_fund": "19",
            "prop_type": "15",
            "prop_category": "1",
            "prop_sector": "[]",
            "prop_assign": "2",
            "accom_total": 90,
            "accom_value": null,
            "accom_date": null,
            "accom_elapse": null,
            "accom_slippage": null,
            "accom_period": null,
            "accom_todate": null,
            "accom_qty": null,
            "accom_amt": null,
            "date": null,
            "status": "CREATED",
            "start": null,
            "target": null,
            "office": "Engineer's Office",
            "assigned": null,
            "appropriation": 300000,
            "abc": 0,
            "status_id": {
              "data": null
            },
            "remarks": "On-going implementation of the project\n90% physical accomplishment as of March 8,2023\n",
            "entry_type": "IMPLEMENTED",
            "bid_date": null,
            "year": 2023,
            "abc_published": null,
            "prop_takers": "PARAGUYA REMIL",
            "prop_infra": null,
            "time_extension": null
          }
        }), db_connection).await,
convert_projects_into_active_model_v2(&json!({
          "id": "1172",
          "attributes": {
            "name": "Construction of Baloyboan Covered Court 9Roofing Structures), Barangay Baloyboan\n",
            "code": "1SAIL23 - 8000 - B2",
            "location": "[\"21\"]",
            "cost": 0,
            "contractor": {
              "data": null
            },
            "duration": null,
            "adjusted": null,
            "total": null,
            "weight": null,
            "prop_sdg": "[]",
            "prop_fund": "19",
            "prop_type": "16",
            "prop_category": "1",
            "prop_sector": "[]",
            "prop_assign": "2",
            "accom_total": 0,
            "accom_value": null,
            "accom_date": null,
            "accom_elapse": null,
            "accom_slippage": null,
            "accom_period": null,
            "accom_todate": null,
            "accom_qty": null,
            "accom_amt": null,
            "date": null,
            "status": "CREATED",
            "start": null,
            "target": null,
            "office": "Engineer's Office",
            "assigned": null,
            "appropriation": 800000,
            "abc": 0,
            "status_id": {
              "data": null
            },
            "remarks": null,
            "entry_type": "IMPLEMENTED",
            "bid_date": null,
            "year": 2023,
            "abc_published": null,
            "prop_takers": null,
            "prop_infra": null,
            "time_extension": null
          }
        }), db_connection).await,
convert_projects_into_active_model_v2(&json!({
          "id": "1173",
          "attributes": {
            "name": "Repair/Renovation of Bogo Barangay Hall, Barangay Bogo\n",
            "code": "1SAIP23 - 8000 - B3",
            "location": "[\"22\"]",
            "cost": 0,
            "contractor": {
              "data": null
            },
            "duration": null,
            "adjusted": null,
            "total": null,
            "weight": null,
            "prop_sdg": "[]",
            "prop_fund": "19",
            "prop_type": "17",
            "prop_category": "3",
            "prop_sector": "[]",
            "prop_assign": "2",
            "accom_total": 5,
            "accom_value": null,
            "accom_date": null,
            "accom_elapse": null,
            "accom_slippage": null,
            "accom_period": null,
            "accom_todate": null,
            "accom_qty": null,
            "accom_amt": null,
            "date": null,
            "status": "CREATED",
            "start": null,
            "target": null,
            "office": "Engineer's Office",
            "assigned": null,
            "appropriation": 800000,
            "abc": 0,
            "status_id": {
              "data": null
            },
            "remarks": null,
            "entry_type": "IMPLEMENTED",
            "bid_date": null,
            "year": 2023,
            "abc_published": null,
            "prop_takers": null,
            "prop_infra": null,
            "time_extension": null
          }
        }), db_connection).await,
convert_projects_into_active_model_v2(&json!({
          "id": "1174",
          "attributes": {
            "name": "Construction of Bomba Covered Court (Roofing Structures), Barangay Bomba\n",
            "code": "1SAIL23-8000-B4",
            "location": "[\"23\"]",
            "cost": 1736000.07,
            "contractor": {
              "data": {
                "id": "2",
                "attributes": {
                  "name": "ALZ Construction",
                  "street_purok": null
                }
              }
            },
            "duration": 75,
            "adjusted": null,
            "total": null,
            "weight": null,
            "prop_sdg": "[]",
            "prop_fund": "19",
            "prop_type": "16",
            "prop_category": "1",
            "prop_sector": "[]",
            "prop_assign": "2",
            "accom_total": 100,
            "accom_value": null,
            "accom_date": null,
            "accom_elapse": null,
            "accom_slippage": null,
            "accom_period": null,
            "accom_todate": null,
            "accom_qty": null,
            "accom_amt": null,
            "date": null,
            "status": "CREATED",
            "start": "2023-07-24",
            "target": "2023-10-07",
            "office": "Engineer's Office",
            "assigned": null,
            "appropriation": 1800000,
            "abc": 0,
            "status_id": {
              "data": null
            },
            "remarks": "100% physically completed based on approved technical plans & specifications\n",
            "entry_type": "IMPLEMENTED",
            "bid_date": null,
            "year": 2023,
            "abc_published": null,
            "prop_takers": null,
            "prop_infra": null,
            "time_extension": null
          }
        }), db_connection).await,
convert_projects_into_active_model_v2(&json!({
          "id": "1175",
          "attributes": {
            "name": "Construction of Gymnasium at Nazareth Area, Barangay Buenavista\n",
            "code": "1SAIP23 - 8000 - B5",
            "location": "[\"4\"]",
            "cost": 0,
            "contractor": {
              "data": null
            },
            "duration": null,
            "adjusted": null,
            "total": null,
            "weight": null,
            "prop_sdg": "[]",
            "prop_fund": "19",
            "prop_type": "18",
            "prop_category": "1",
            "prop_sector": "[]",
            "prop_assign": "2",
            "accom_total": 0,
            "accom_value": null,
            "accom_date": null,
            "accom_elapse": null,
            "accom_slippage": null,
            "accom_period": null,
            "accom_todate": null,
            "accom_qty": null,
            "accom_amt": null,
            "date": null,
            "status": "CREATED",
            "start": null,
            "target": null,
            "office": "Engineer's Office",
            "assigned": null,
            "appropriation": 1800000,
            "abc": 0,
            "status_id": {
              "data": null
            },
            "remarks": null,
            "entry_type": "IMPLEMENTED",
            "bid_date": null,
            "year": 2023,
            "abc_published": null,
            "prop_takers": null,
            "prop_infra": null,
            "time_extension": null
          }
        }), db_connection).await,
convert_projects_into_active_model_v2(&json!({
          "id": "1176",
          "attributes": {
            "name": "Construction of Covered Court (Phase III), Barangay Dampalan\n",
            "code": "1SAIP23 - 8000 - B6",
            "location": "[\"26\"]",
            "cost": 474664.72,
            "contractor": {
              "data": {
                "id": "19",
                "attributes": {
                  "name": "Jenrich Construction",
                  "street_purok": null
                }
              }
            },
            "duration": null,
            "adjusted": null,
            "total": null,
            "weight": null,
            "prop_sdg": "[]",
            "prop_fund": "19",
            "prop_type": "16",
            "prop_category": "1",
            "prop_sector": "[]",
            "prop_assign": "2",
            "accom_total": 0,
            "accom_value": null,
            "accom_date": null,
            "accom_elapse": null,
            "accom_slippage": null,
            "accom_period": null,
            "accom_todate": null,
            "accom_qty": null,
            "accom_amt": null,
            "date": null,
            "status": "CREATED",
            "start": null,
            "target": null,
            "office": "Engineer's Office",
            "assigned": null,
            "appropriation": 500000,
            "abc": 0,
            "status_id": {
              "data": null
            },
            "remarks": null,
            "entry_type": "IMPLEMENTED",
            "bid_date": null,
            "year": 2023,
            "abc_published": null,
            "prop_takers": null,
            "prop_infra": null,
            "time_extension": null
          }
        }), db_connection).await,
convert_projects_into_active_model_v2(&json!({
          "id": "1177",
          "attributes": {
            "name": "Construction of Multi-Purpose Building (Phase II), Barangay Dampalan\n",
            "code": "1SAIL23 - 8000 - B7",
            "location": "[\"26\"]",
            "cost": 483989.73,
            "contractor": {
              "data": {
                "id": "19",
                "attributes": {
                  "name": "Jenrich Construction",
                  "street_purok": null
                }
              }
            },
            "duration": null,
            "adjusted": null,
            "total": null,
            "weight": null,
            "prop_sdg": "[]",
            "prop_fund": "19",
            "prop_type": "5",
            "prop_category": "1",
            "prop_sector": "[]",
            "prop_assign": "2",
            "accom_total": 100,
            "accom_value": null,
            "accom_date": null,
            "accom_elapse": null,
            "accom_slippage": null,
            "accom_period": null,
            "accom_todate": null,
            "accom_qty": null,
            "accom_amt": null,
            "date": null,
            "status": "CREATED",
            "start": null,
            "target": null,
            "office": "Engineer's Office",
            "assigned": null,
            "appropriation": 500000,
            "abc": 0,
            "status_id": {
              "data": null
            },
            "remarks": null,
            "entry_type": "IMPLEMENTED",
            "bid_date": null,
            "year": 2023,
            "abc_published": null,
            "prop_takers": null,
            "prop_infra": null,
            "time_extension": null
          }
        }), db_connection).await,
convert_projects_into_active_model_v2(&json!({
          "id": "1178",
          "attributes": {
            "name": "Construction of Day Care Center Building, Barangay Dampalan",
            "code": "1SAIL23 - 8000 - B8",
            "location": "[\"26\"]",
            "cost": 0,
            "contractor": {
              "data": null
            },
            "duration": null,
            "adjusted": null,
            "total": null,
            "weight": null,
            "prop_sdg": "[]",
            "prop_fund": "5",
            "prop_type": null,
            "prop_category": null,
            "prop_sector": "[]",
            "prop_assign": null,
            "accom_total": 0,
            "accom_value": null,
            "accom_date": null,
            "accom_elapse": null,
            "accom_slippage": null,
            "accom_period": null,
            "accom_todate": null,
            "accom_qty": null,
            "accom_amt": null,
            "date": null,
            "status": "CREATED",
            "start": null,
            "target": null,
            "office": "Engineer's Office",
            "assigned": null,
            "appropriation": 500000,
            "abc": 0,
            "status_id": {
              "data": null
            },
            "remarks": null,
            "entry_type": "IMPLEMENTED",
            "bid_date": null,
            "year": 2023,
            "abc_published": null,
            "prop_takers": null,
            "prop_infra": null,
            "time_extension": null
          }
        }), db_connection).await,
convert_projects_into_active_model_v2(&json!({
          "id": "1179",
          "attributes": {
            "name": "Construction of Danlugan Wet Market Phase IV, Barangay Danlugan\n",
            "code": "1SAIL23 - 8000 - B9",
            "location": "[\"27\"]",
            "cost": 970433.79,
            "contractor": {
              "data": {
                "id": "6",
                "attributes": {
                  "name": "DM Ventures",
                  "street_purok": null
                }
              }
            },
            "duration": null,
            "adjusted": null,
            "total": null,
            "weight": null,
            "prop_sdg": "[]",
            "prop_fund": "17",
            "prop_type": "3",
            "prop_category": "1",
            "prop_sector": "[]",
            "prop_assign": "2",
            "accom_total": 85,
            "accom_value": null,
            "accom_date": null,
            "accom_elapse": null,
            "accom_slippage": null,
            "accom_period": null,
            "accom_todate": null,
            "accom_qty": null,
            "accom_amt": null,
            "date": null,
            "status": "CREATED",
            "start": null,
            "target": null,
            "office": "Engineer's Office",
            "assigned": null,
            "appropriation": 1000000,
            "abc": 0,
            "status_id": {
              "data": null
            },
            "remarks": "On-going implementation of the project\n",
            "entry_type": "IMPLEMENTED",
            "bid_date": null,
            "year": 2023,
            "abc_published": null,
            "prop_takers": "CAPT. DACAL",
            "prop_infra": null,
            "time_extension": null
          }
        }), db_connection).await,
        convert_projects_into_active_model_v2(&json!({
          "id": "1180",
          "attributes": {
            "name": "Construction of Covered Court at Prk Fatima, Dao\n",
            "code": "1SAIL23-8000-B10",
            "location": "[\"5\"]",
            "cost": 1103883.81,
            "contractor": {
              "data": {
                "id": "8",
                "attributes": {
                  "name": "JAMT Construction",
                  "street_purok": null
                }
              }
            },
            "duration": null,
            "adjusted": null,
            "total": null,
            "weight": null,
            "prop_sdg": "[]",
            "prop_fund": "19",
            "prop_type": "16",
            "prop_category": "1",
            "prop_sector": "[]",
            "prop_assign": "2",
            "accom_total": 0,
            "accom_value": null,
            "accom_date": null,
            "accom_elapse": null,
            "accom_slippage": null,
            "accom_period": null,
            "accom_todate": null,
            "accom_qty": null,
            "accom_amt": null,
            "date": null,
            "status": "CREATED",
            "start": null,
            "target": null,
            "office": "Engineer's Office",
            "assigned": null,
            "appropriation": 1150000,
            "abc": 1104713.29,
            "status_id": {
              "data": null
            },
            "remarks": "Bided but not yet started",
            "entry_type": "IMPLEMENTED",
            "bid_date": null,
            "year": 2023,
            "abc_published": null,
            "prop_takers": "AU-AU BASAS",
            "prop_infra": null,
            "time_extension": null
          }
        }), db_connection).await,
convert_projects_into_active_model_v2(&json!({
          "id": "1181",
          "attributes": {
            "name": "Concreting of Flooring at Gymnasium, Barangay Ditoray\n",
            "code": "1SAIL23 - 8000 - B11",
            "location": "[\"30\"]",
            "cost": 0,
            "contractor": {
              "data": null
            },
            "duration": null,
            "adjusted": null,
            "total": null,
            "weight": null,
            "prop_sdg": "[]",
            "prop_fund": "19",
            "prop_type": "18",
            "prop_category": "2",
            "prop_sector": "[]",
            "prop_assign": "2",
            "accom_total": 0,
            "accom_value": null,
            "accom_date": null,
            "accom_elapse": null,
            "accom_slippage": null,
            "accom_period": null,
            "accom_todate": null,
            "accom_qty": null,
            "accom_amt": null,
            "date": null,
            "status": "CREATED",
            "start": null,
            "target": null,
            "office": "Engineer's Office",
            "assigned": null,
            "appropriation": 400000,
            "abc": 0,
            "status_id": {
              "data": null
            },
            "remarks": null,
            "entry_type": "IMPLEMENTED",
            "bid_date": null,
            "year": 2023,
            "abc_published": null,
            "prop_takers": null,
            "prop_infra": null,
            "time_extension": null
          }
        }), db_connection).await,
convert_projects_into_active_model_v2(&json!({
          "id": "1182",
          "attributes": {
            "name": "Construction Of Gatas Barangay Hall (Phase II), Barangay Gatas\n",
            "code": "1SAIL23 - 8000 - B12",
            "location": "[\"7\"]",
            "cost": 0,
            "contractor": {
              "data": null
            },
            "duration": null,
            "adjusted": null,
            "total": null,
            "weight": null,
            "prop_sdg": "[]",
            "prop_fund": "19",
            "prop_type": "17",
            "prop_category": "1",
            "prop_sector": "[]",
            "prop_assign": "2",
            "accom_total": 0,
            "accom_value": null,
            "accom_date": null,
            "accom_elapse": null,
            "accom_slippage": null,
            "accom_period": null,
            "accom_todate": null,
            "accom_qty": null,
            "accom_amt": null,
            "date": null,
            "status": "CREATED",
            "start": null,
            "target": null,
            "office": "Engineer's Office",
            "assigned": null,
            "appropriation": 1500000,
            "abc": 0,
            "status_id": {
              "data": null
            },
            "remarks": null,
            "entry_type": "IMPLEMENTED",
            "bid_date": null,
            "year": 2023,
            "abc_published": null,
            "prop_takers": null,
            "prop_infra": null,
            "time_extension": null
          }
        }), db_connection).await,
convert_projects_into_active_model_v2(&json!({
          "id": "1183",
          "attributes": {
            "name": "Construction of Multi-Purpose Building (Phase III), Barangay Gubac\n",
            "code": "1SAIL23 - 8000 - B13",
            "location": "[\"31\"]",
            "cost": 0,
            "contractor": {
              "data": null
            },
            "duration": null,
            "adjusted": null,
            "total": null,
            "weight": null,
            "prop_sdg": "[]",
            "prop_fund": "19",
            "prop_type": "5",
            "prop_category": "1",
            "prop_sector": "[]",
            "prop_assign": "2",
            "accom_total": 0,
            "accom_value": null,
            "accom_date": null,
            "accom_elapse": null,
            "accom_slippage": null,
            "accom_period": null,
            "accom_todate": null,
            "accom_qty": null,
            "accom_amt": null,
            "date": null,
            "status": "CREATED",
            "start": null,
            "target": null,
            "office": "Engineer's Office",
            "assigned": null,
            "appropriation": 600000,
            "abc": 0,
            "status_id": {
              "data": null
            },
            "remarks": null,
            "entry_type": "IMPLEMENTED",
            "bid_date": null,
            "year": 2023,
            "abc_published": null,
            "prop_takers": null,
            "prop_infra": null,
            "time_extension": null
          }
        }), db_connection).await,
convert_projects_into_active_model_v2(&json!({
          "id": "1184",
          "attributes": {
            "name": "Construction of Freedom Stage at Barangay Gubang\n",
            "code": "1SAIL23 - 8000 - B14",
            "location": "[\"32\"]",
            "cost": 0,
            "contractor": {
              "data": null
            },
            "duration": null,
            "adjusted": null,
            "total": null,
            "weight": null,
            "prop_sdg": "[]",
            "prop_fund": "19",
            "prop_type": "20",
            "prop_category": "1",
            "prop_sector": "[]",
            "prop_assign": "2",
            "accom_total": 100,
            "accom_value": null,
            "accom_date": null,
            "accom_elapse": null,
            "accom_slippage": null,
            "accom_period": null,
            "accom_todate": null,
            "accom_qty": null,
            "accom_amt": null,
            "date": null,
            "status": "CREATED",
            "start": null,
            "target": null,
            "office": "Engineer's Office",
            "assigned": null,
            "appropriation": 500000,
            "abc": 0,
            "status_id": {
              "data": null
            },
            "remarks": null,
            "entry_type": "IMPLEMENTED",
            "bid_date": null,
            "year": 2023,
            "abc_published": null,
            "prop_takers": null,
            "prop_infra": null,
            "time_extension": null
          }
        }), db_connection).await,
convert_projects_into_active_model_v2(&json!({
          "id": "1185",
          "attributes": {
            "name": "Construction of Barangay Stage & Bleacher at Barangay Kagawasan\n",
            "code": "1SAIL23-8000-B15",
            "location": "[\"33\"]",
            "cost": 0,
            "contractor": {
              "data": null
            },
            "duration": null,
            "adjusted": null,
            "total": null,
            "weight": null,
            "prop_sdg": "[]",
            "prop_fund": "19",
            "prop_type": "20",
            "prop_category": "1",
            "prop_sector": "[]",
            "prop_assign": "2",
            "accom_total": 100,
            "accom_value": null,
            "accom_date": null,
            "accom_elapse": null,
            "accom_slippage": null,
            "accom_period": null,
            "accom_todate": null,
            "accom_qty": null,
            "accom_amt": null,
            "date": null,
            "status": "CREATED",
            "start": null,
            "target": null,
            "office": "Engineer's Office",
            "assigned": null,
            "appropriation": 500000,
            "abc": 0,
            "status_id": {
              "data": null
            },
            "remarks": "100% physically completed based on approved technical plans & specifications\n",
            "entry_type": "IMPLEMENTED",
            "bid_date": null,
            "year": 2023,
            "abc_published": null,
            "prop_takers": null,
            "prop_infra": null,
            "time_extension": null
          }
        }), db_connection).await,
convert_projects_into_active_model_v2(&json!({
          "id": "1186",
          "attributes": {
            "name": "Construction of Four (4) units of Purok Halls, Barangay Kalasan\n",
            "code": "1SAIL23 - 8000 - B16",
            "location": "[\"35\"]",
            "cost": 684454.19,
            "contractor": {
              "data": {
                "id": "8",
                "attributes": {
                  "name": "JAMT Construction",
                  "street_purok": null
                }
              }
            },
            "duration": null,
            "adjusted": null,
            "total": null,
            "weight": null,
            "prop_sdg": "[]",
            "prop_fund": "19",
            "prop_type": "21",
            "prop_category": "1",
            "prop_sector": "[]",
            "prop_assign": "2",
            "accom_total": 70,
            "accom_value": null,
            "accom_date": null,
            "accom_elapse": null,
            "accom_slippage": null,
            "accom_period": null,
            "accom_todate": null,
            "accom_qty": null,
            "accom_amt": null,
            "date": null,
            "status": "CREATED",
            "start": null,
            "target": null,
            "office": "Engineer's Office",
            "assigned": null,
            "appropriation": 700000,
            "abc": 0,
            "status_id": {
              "data": null
            },
            "remarks": "On-going implementation of the project\n",
            "entry_type": "IMPLEMENTED",
            "bid_date": null,
            "year": 2023,
            "abc_published": null,
            "prop_takers": null,
            "prop_infra": null,
            "time_extension": null
          }
        }), db_connection).await,
convert_projects_into_active_model_v2(&json!({
          "id": "1187",
          "attributes": {
            "name": "Construction of Six (6) units of Purok Halls at Barangay Lumad\n",
            "code": "1SAIL23 - 8000 - B17",
            "location": "[\"43\"]",
            "cost": 0,
            "contractor": {
              "data": null
            },
            "duration": null,
            "adjusted": null,
            "total": null,
            "weight": null,
            "prop_sdg": "[]",
            "prop_fund": "19",
            "prop_type": "21",
            "prop_category": "1",
            "prop_sector": "[]",
            "prop_assign": "2",
            "accom_total": 0,
            "accom_value": null,
            "accom_date": null,
            "accom_elapse": null,
            "accom_slippage": null,
            "accom_period": null,
            "accom_todate": null,
            "accom_qty": null,
            "accom_amt": null,
            "date": null,
            "status": "CREATED",
            "start": null,
            "target": null,
            "office": "Engineer's Office",
            "assigned": null,
            "appropriation": 500000,
            "abc": 0,
            "status_id": {
              "data": null
            },
            "remarks": null,
            "entry_type": "IMPLEMENTED",
            "bid_date": null,
            "year": 2023,
            "abc_published": null,
            "prop_takers": null,
            "prop_infra": null,
            "time_extension": null
          }
        }), db_connection).await,
convert_projects_into_active_model_v2(&json!({
          "id": "1188",
          "attributes": {
            "name": "Improvement of Covered Court at Barangay Lumbia\n",
            "code": "1SAIL23 - 8000 - B18",
            "location": "[\"9\"]",
            "cost": 0,
            "contractor": {
              "data": null
            },
            "duration": null,
            "adjusted": null,
            "total": null,
            "weight": null,
            "prop_sdg": "[]",
            "prop_fund": "19",
            "prop_type": "16",
            "prop_category": "4",
            "prop_sector": "[]",
            "prop_assign": "2",
            "accom_total": 0,
            "accom_value": null,
            "accom_date": null,
            "accom_elapse": null,
            "accom_slippage": null,
            "accom_period": null,
            "accom_todate": null,
            "accom_qty": null,
            "accom_amt": null,
            "date": null,
            "status": "CREATED",
            "start": null,
            "target": null,
            "office": "Engineer's Office",
            "assigned": null,
            "appropriation": 1000000,
            "abc": 0,
            "status_id": {
              "data": null
            },
            "remarks": null,
            "entry_type": "IMPLEMENTED",
            "bid_date": null,
            "year": 2023,
            "abc_published": null,
            "prop_takers": null,
            "prop_infra": null,
            "time_extension": null
          }
        }), db_connection).await,
convert_projects_into_active_model_v2(&json!({
          "id": "1189",
          "attributes": {
            "name": "Construction of Purok Shed at Zone 5, Barangay Muricay\n",
            "code": "1SAIL23-8000-B19",
            "location": "[\"46\"]",
            "cost": 199595.63,
            "contractor": {
              "data": {
                "id": "1",
                "attributes": {
                  "name": "CTG Construction",
                  "street_purok": "Purok 1"
                }
              }
            },
            "duration": 60,
            "adjusted": null,
            "total": null,
            "weight": null,
            "prop_sdg": "[]",
            "prop_fund": "19",
            "prop_type": "21",
            "prop_category": "1",
            "prop_sector": "[]",
            "prop_assign": "2",
            "accom_total": 100,
            "accom_value": null,
            "accom_date": null,
            "accom_elapse": null,
            "accom_slippage": null,
            "accom_period": null,
            "accom_todate": null,
            "accom_qty": null,
            "accom_amt": null,
            "date": null,
            "status": "CREATED",
            "start": "2023-10-30",
            "target": "2023-12-29",
            "office": "Engineer's Office",
            "assigned": null,
            "appropriation": 200000,
            "abc": 193882.07,
            "status_id": {
              "data": null
            },
            "remarks": "100% physically completed based on approved technical plans & specifications\n\n* ACTUAL Date Completed:12/28/2023",
            "entry_type": "IMPLEMENTED",
            "bid_date": null,
            "year": 2023,
            "abc_published": null,
            "prop_takers": "KAG. MURICAY",
            "prop_infra": null,
            "time_extension": null
          }
        }), db_connection).await,
        convert_projects_into_active_model_v2(&json!({
          "id": "1190",
          "attributes": {
            "name": "Completion of San Jose Barangay Hall, Barangay San Jose",
            "code": "1SAIL23-8000-B20",
            "location": "[\"12\"]",
            "cost": 773400.05,
            "contractor": {
              "data": {
                "id": "19",
                "attributes": {
                  "name": "Jenrich Construction",
                  "street_purok": null
                }
              }
            },
            "duration": 90,
            "adjusted": null,
            "total": null,
            "weight": null,
            "prop_sdg": "[]",
            "prop_fund": "19",
            "prop_type": "17",
            "prop_category": "4",
            "prop_sector": "[]",
            "prop_assign": "2",
            "accom_total": 100,
            "accom_value": null,
            "accom_date": null,
            "accom_elapse": null,
            "accom_slippage": null,
            "accom_period": null,
            "accom_todate": null,
            "accom_qty": null,
            "accom_amt": null,
            "date": null,
            "status": "CREATED",
            "start": "2023-07-17",
            "target": "2023-10-15",
            "office": "Engineer's Office",
            "assigned": null,
            "appropriation": 800000,
            "abc": 0,
            "status_id": {
              "data": null
            },
            "remarks": "\"* 100% physically completed based on approved technical plans & specifications\n* w/ time extension as per approved SO No.1 \n* Adjusted Targe Date of Completion: Oct. 2, 2023\"\n* DATE COMPLETED: 10/2/2023",
            "entry_type": "IMPLEMENTED",
            "bid_date": null,
            "year": 2023,
            "abc_published": null,
            "prop_takers": "CAPT. ARAO-ARAO",
            "prop_infra": null,
            "time_extension": null
          }
        }), db_connection).await,
convert_projects_into_active_model_v2(&json!({
          "id": "1191",
          "attributes": {
            "name": "Construction of Ticket Booth at IBT Annex, Barangay Sta Lucia\n",
            "code": "1SAIL23 - 8000 - B21",
            "location": "[\"14\"]",
            "cost": 0,
            "contractor": {
              "data": null
            },
            "duration": null,
            "adjusted": null,
            "total": null,
            "weight": null,
            "prop_sdg": "[]",
            "prop_fund": "19",
            "prop_type": "",
            "prop_category": "1",
            "prop_sector": "[]",
            "prop_assign": null,
            "accom_total": 0,
            "accom_value": null,
            "accom_date": null,
            "accom_elapse": null,
            "accom_slippage": null,
            "accom_period": null,
            "accom_todate": null,
            "accom_qty": null,
            "accom_amt": null,
            "date": null,
            "status": "CREATED",
            "start": null,
            "target": null,
            "office": "Engineer's Office",
            "assigned": null,
            "appropriation": 150000,
            "abc": 0,
            "status_id": {
              "data": null
            },
            "remarks": null,
            "entry_type": "IMPLEMENTED",
            "bid_date": null,
            "year": 2023,
            "abc_published": null,
            "prop_takers": null,
            "prop_infra": null,
            "time_extension": null
          }
        }), db_connection).await,
convert_projects_into_active_model_v2(&json!({
          "id": "1192",
          "attributes": {
            "name": "Construction of Covered Court at Purok Pag-Asa (Banker Village), Sto Nino",
            "code": "1SAIL23 - 8000 - B22",
            "location": "[\"17\"]",
            "cost": 1946290.56,
            "contractor": {
              "data": {
                "id": "15",
                "attributes": {
                  "name": "Aradel Construction",
                  "street_purok": null
                }
              }
            },
            "duration": null,
            "adjusted": null,
            "total": null,
            "weight": null,
            "prop_sdg": "[]",
            "prop_fund": "19",
            "prop_type": "16",
            "prop_category": "1",
            "prop_sector": "[]",
            "prop_assign": "2",
            "accom_total": 5,
            "accom_value": null,
            "accom_date": null,
            "accom_elapse": null,
            "accom_slippage": null,
            "accom_period": null,
            "accom_todate": null,
            "accom_qty": null,
            "accom_amt": null,
            "date": null,
            "status": "CREATED",
            "start": null,
            "target": null,
            "office": "Engineer's Office",
            "assigned": null,
            "appropriation": 2000000,
            "abc": 0,
            "status_id": {
              "data": null
            },
            "remarks": "5% PHYSICAL ACCOMPLISHMENT AS OF NOV. 3, 2023",
            "entry_type": "IMPLEMENTED",
            "bid_date": null,
            "year": 2023,
            "abc_published": null,
            "prop_takers": null,
            "prop_infra": null,
            "time_extension": null
          }
        }), db_connection).await,
convert_projects_into_active_model_v2(&json!({
          "id": "1193",
          "attributes": {
            "name": "Completion of Barangay Hall, Sto. Nino\n",
            "code": "1SAIL23-8000-B23",
            "location": "[\"17\"]",
            "cost": 483397.51,
            "contractor": {
              "data": {
                "id": "10",
                "attributes": {
                  "name": "RSQ Construction",
                  "street_purok": null
                }
              }
            },
            "duration": 60,
            "adjusted": null,
            "total": null,
            "weight": null,
            "prop_sdg": "[]",
            "prop_fund": "19",
            "prop_type": "17",
            "prop_category": "4",
            "prop_sector": "[]",
            "prop_assign": "2",
            "accom_total": 100,
            "accom_value": null,
            "accom_date": null,
            "accom_elapse": null,
            "accom_slippage": null,
            "accom_period": null,
            "accom_todate": null,
            "accom_qty": null,
            "accom_amt": null,
            "date": null,
            "status": "CREATED",
            "start": "2023-07-31",
            "target": "2023-09-29",
            "office": "Engineer's Office",
            "assigned": null,
            "appropriation": 500000,
            "abc": 0,
            "status_id": {
              "data": null
            },
            "remarks": "\"* 100% physically completed based on approved plans & specifications\n* with time extension as per approved   SUSPENSION ORDER  due to variation order\n* Adjusted Target date of Completion: March 30, 2024\n* Actual Completion Date:March 25, 2024\"\n",
            "entry_type": "IMPLEMENTED",
            "bid_date": null,
            "year": 2023,
            "abc_published": null,
            "prop_takers": null,
            "prop_infra": null,
            "time_extension": null
          }
        }), db_connection).await,
convert_projects_into_active_model_v2(&json!({
          "id": "1194",
          "attributes": {
            "name": "Construction of Covered Court at Co Tek Chun, Barangay Tiguma\n",
            "code": "1SAIL23-8000-B24",
            "location": "[\"18\"]",
            "cost": 1997849.25,
            "contractor": {
              "data": {
                "id": "1",
                "attributes": {
                  "name": "CTG Construction",
                  "street_purok": "Purok 1"
                }
              }
            },
            "duration": 90,
            "adjusted": null,
            "total": null,
            "weight": null,
            "prop_sdg": "[]",
            "prop_fund": "19",
            "prop_type": "16",
            "prop_category": "1",
            "prop_sector": "[]",
            "prop_assign": "2",
            "accom_total": 100,
            "accom_value": null,
            "accom_date": null,
            "accom_elapse": null,
            "accom_slippage": null,
            "accom_period": null,
            "accom_todate": null,
            "accom_qty": null,
            "accom_amt": null,
            "date": null,
            "status": "CREATED",
            "start": "2023-10-02",
            "target": "2023-12-31",
            "office": "Engineer's Office",
            "assigned": null,
            "appropriation": 2000000,
            "abc": 0,
            "status_id": {
              "data": null
            },
            "remarks": "\"* 100% physically completed\n* W/ Time extension as per approved SO. NO 1\n* Adjusted Target date of Completion: 3/2/2024\n* Actual Completion Date: 3/1/2024\"",
            "entry_type": "IMPLEMENTED",
            "bid_date": null,
            "year": 2023,
            "abc_published": null,
            "prop_takers": "CTG-HERME GUINEA",
            "prop_infra": null,
            "time_extension": null
          }
        }), db_connection).await,
convert_projects_into_active_model_v2(&json!({
          "id": "1195",
          "attributes": {
            "name": " Improvement of Covered Court at PAGSCI, Barangay Tuburan\n",
            "code": "1SAIL23 - 8000 - B25",
            "location": "[\"19\"]",
            "cost": 0,
            "contractor": {
              "data": null
            },
            "duration": null,
            "adjusted": null,
            "total": null,
            "weight": null,
            "prop_sdg": "[]",
            "prop_fund": "19",
            "prop_type": "16",
            "prop_category": "4",
            "prop_sector": "[]",
            "prop_assign": "2",
            "accom_total": 0,
            "accom_value": null,
            "accom_date": null,
            "accom_elapse": null,
            "accom_slippage": null,
            "accom_period": null,
            "accom_todate": null,
            "accom_qty": null,
            "accom_amt": null,
            "date": null,
            "status": "CREATED",
            "start": null,
            "target": null,
            "office": "Engineer's Office",
            "assigned": null,
            "appropriation": 1200000,
            "abc": 0,
            "status_id": {
              "data": null
            },
            "remarks": null,
            "entry_type": "IMPLEMENTED",
            "bid_date": null,
            "year": 2023,
            "abc_published": null,
            "prop_takers": null,
            "prop_infra": null,
            "time_extension": null
          }
        }), db_connection).await,
convert_projects_into_active_model_v2(&json!({
          "id": "1196",
          "attributes": {
            "name": "Completion of Day Care Center (Phase II) Barangay Upper Sibatang\n",
            "code": "1SAIL23 - 8000 - B26",
            "location": "[\"53\"]",
            "cost": 0,
            "contractor": {
              "data": null
            },
            "duration": null,
            "adjusted": null,
            "total": null,
            "weight": null,
            "prop_sdg": "[]",
            "prop_fund": "19",
            "prop_type": "19",
            "prop_category": "1",
            "prop_sector": "[]",
            "prop_assign": "2",
            "accom_total": 80,
            "accom_value": null,
            "accom_date": null,
            "accom_elapse": null,
            "accom_slippage": null,
            "accom_period": null,
            "accom_todate": null,
            "accom_qty": null,
            "accom_amt": null,
            "date": null,
            "status": "CREATED",
            "start": null,
            "target": null,
            "office": "Engineer's Office",
            "assigned": null,
            "appropriation": 400000,
            "abc": 0,
            "status_id": {
              "data": null
            },
            "remarks": "80% PHYSICAL ACCOMPLISHMENT as of Nov. 10, 2023",
            "entry_type": "IMPLEMENTED",
            "bid_date": null,
            "year": 2023,
            "abc_published": null,
            "prop_takers": null,
            "prop_infra": null,
            "time_extension": null
          }
        }), db_connection).await
        ];

        projects::Entity::insert_many(make_add).exec(db_connection).await.unwrap();

        Ok(())
    }
}