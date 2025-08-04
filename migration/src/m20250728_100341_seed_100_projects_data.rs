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
              "id": "172",
              "attributes": {
                "name": "Construction of Baywalk Commercial Stalls",
                "code": "2SAIP22 - 8000R - C1c2",
                "location": null,
                "cost": 0,
                "contractor": {
                  "data": null
                },
                "duration": null,
                "adjusted": null,
                "total": null,
                "weight": null,
                "prop_sdg": null,
                "prop_fund": "6",
                "prop_type": null,
                "prop_category": null,
                "prop_sector": null,
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
                "appropriation": 10000000,
                "abc": 0,
                "status_id": {
                  "data": null
                },
                "remarks": null,
                "entry_type": "IMPLEMENTED",
                "bid_date": null,
                "year": 2022,
                "abc_published": null,
                "prop_takers": null,
                "prop_infra": null,
                "time_extension": null
              }
            }), db_connection).await,
    convert_projects_into_active_model_v2(&json!({
              "id": "173",
              "attributes": {
                "name": "Development of Pagadian City Water Park Continuation -2SAIP22-8000R-C1c3 (10M) and  Development of Water Park at  Boulevard , Barangay  Santiago-3SAIP20-8000-B2b(1.5m)",
                "code": "\"2SAIP22-8000R-C1c3 (joint fund w/   3SAIP20-8000-B2b)\"",
                "location": "[\"13\"]",
                "cost": 11239854.8,
                "contractor": {
                  "data": {
                    "id": "13",
                    "attributes": {
                      "name": "Glenson Construction",
                      "street_purok": null
                    }
                  }
                },
                "duration": 240,
                "adjusted": null,
                "total": null,
                "weight": null,
                "prop_sdg": "[]",
                "prop_fund": "6",
                "prop_type": "26",
                "prop_category": "1",
                "prop_sector": "[]",
                "prop_assign": "2",
                "accom_total": 32,
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
                "start": "2024-07-05",
                "target": "2025-03-02",
                "office": "Engineer's Office",
                "assigned": null,
                "appropriation": 11500000,
                "abc": 11263921.32,
                "status_id": {
                  "data": null
                },
                "remarks": "On-going EMBANKMENT undertaken by ADMIN\n",
                "entry_type": "IMPLEMENTED",
                "bid_date": null,
                "year": 2022,
                "abc_published": null,
                "prop_takers": "EDWIN DY & PATRICK WEE",
                "prop_infra": null,
                "time_extension": null
              }
            }), db_connection).await,
    convert_projects_into_active_model_v2(&json!({
              "id": "174",
              "attributes": {
                "name": "Rehab of Road (Access Road) Leading to Lison Valley Tribal Village",
                "code": "4SAIP22-8000R-B1-1a2",
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
                "prop_fund": "8",
                "prop_type": "1",
                "prop_category": "3",
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
                "appropriation": 1500000,
                "abc": 0,
                "status_id": {
                  "data": null
                },
                "remarks": null,
                "entry_type": "IMPLEMENTED",
                "bid_date": null,
                "year": 2022,
                "abc_published": null,
                "prop_takers": null,
                "prop_infra": null,
                "time_extension": null
              }
            }), db_connection).await,
    convert_projects_into_active_model_v2(&json!({
              "id": "175",
              "attributes": {
                "name": " Construction of City Government Warehouse Building Phase 1 (RF21-8000-3B-5 , p1,029,500.00) and Construction of Structures  at Bagong Silang Dumpsite (4SAIP22-8000R-B1-1b2, P320T)",
                "code": "4SAIP22-8000R-B1-1b2/ joint fund w/RF21-8000-3B-5/",
                "location": "[\"1\"]",
                "cost": 1320798.48,
                "contractor": {
                  "data": {
                    "id": "20",
                    "attributes": {
                      "name": "Maqueserg Construction",
                      "street_purok": null
                    }
                  }
                },
                "duration": 152,
                "adjusted": null,
                "total": null,
                "weight": null,
                "prop_sdg": "[]",
                "prop_fund": "8",
                "prop_type": "26",
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
                "start": "2023-09-26",
                "target": "2024-02-25",
                "office": "Engineer's Office",
                "assigned": null,
                "appropriation": 1349500,
                "abc": 1322552.33,
                "status_id": {
                  "data": null
                },
                "remarks": "\"* 100% physically completed based on approved technical plans & specifications\n* Adjusted Target date of Completion:  April 25, 2024\n* Actual Completion Date: 2/22/2024\"\n",
                "entry_type": "IMPLEMENTED",
                "bid_date": null,
                "year": 2022,
                "abc_published": null,
                "prop_takers": "MARIANETTE MARCABAN",
                "prop_infra": null,
                "time_extension": null
              }
            }), db_connection).await,
    convert_projects_into_active_model_v2(&json!({
              "id": "176",
              "attributes": {
                "name": "Construction of Senior Citizen Building (Completion)",
                "code": "SCPWD22 - 3000 -A3",
                "location": null,
                "cost": 0,
                "contractor": {
                  "data": null
                },
                "duration": null,
                "adjusted": null,
                "total": null,
                "weight": null,
                "prop_sdg": null,
                "prop_fund": "10",
                "prop_type": null,
                "prop_category": null,
                "prop_sector": null,
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
                "appropriation": 5000000,
                "abc": 0,
                "status_id": {
                  "data": null
                },
                "remarks": null,
                "entry_type": "IMPLEMENTED",
                "bid_date": null,
                "year": 2022,
                "abc_published": null,
                "prop_takers": null,
                "prop_infra": null,
                "time_extension": null
              }
            }), db_connection).await,
    convert_projects_into_active_model_v2(&json!({
              "id": "177",
              "attributes": {
                "name": "Construction of Proposed PWD Office Building (Completion)",
                "code": "SCPWD22 - 3000 - B5",
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
                "prop_fund": "10",
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
                "appropriation": 5180000,
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
              "id": "178",
              "attributes": {
                "name": "Materials Laboratory\n",
                "code": "RF21 - 1000 - 1 -13 -c",
                "location": null,
                "cost": 0,
                "contractor": {
                  "data": null
                },
                "duration": null,
                "adjusted": null,
                "total": null,
                "weight": null,
                "prop_sdg": null,
                "prop_fund": "1",
                "prop_type": null,
                "prop_category": null,
                "prop_sector": null,
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
                "appropriation": 100000,
                "abc": 0,
                "status_id": {
                  "data": null
                },
                "remarks": null,
                "entry_type": "IMPLEMENTED",
                "bid_date": null,
                "year": 2021,
                "abc_published": null,
                "prop_takers": null,
                "prop_infra": null,
                "time_extension": null
              }
            }), db_connection).await,
    convert_projects_into_active_model_v2(&json!({
              "id": "179",
              "attributes": {
                "name": "Construction of Police Outpost\n",
                "code": "RF21 - 1000 - 2A - 1J",
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
                "prop_fund": "1",
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
                "appropriation": 2800000,
                "abc": 0,
                "status_id": {
                  "data": null
                },
                "remarks": null,
                "entry_type": "IMPLEMENTED",
                "bid_date": null,
                "year": 2021,
                "abc_published": null,
                "prop_takers": null,
                "prop_infra": null,
                "time_extension": null
              }
            }), db_connection).await,
            convert_projects_into_active_model_v2(&json!({
              "id": "180",
              "attributes": {
                "name": "Rehab/Repair/Maint of Barangay Roads\n",
                "code": "RF21 - 8000 - 3A - 2",
                "location": null,
                "cost": 0,
                "contractor": {
                  "data": null
                },
                "duration": null,
                "adjusted": null,
                "total": null,
                "weight": null,
                "prop_sdg": null,
                "prop_fund": "1",
                "prop_type": null,
                "prop_category": null,
                "prop_sector": null,
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
                "appropriation": 1000000,
                "abc": 0,
                "status_id": {
                  "data": null
                },
                "remarks": null,
                "entry_type": "IMPLEMENTED",
                "bid_date": null,
                "year": 2021,
                "abc_published": null,
                "prop_takers": null,
                "prop_infra": null,
                "time_extension": null
              }
            }), db_connection).await,
    convert_projects_into_active_model_v2(&json!({
              "id": "181",
              "attributes": {
                "name": "Concreting of Datagan - Kindes Cave - Quarry Site Brgy Road One Side\n",
                "code": "RF21 - 8000 - 3A - 3",
                "location": "[\"28\"]",
                "cost": 9679694.65,
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
                "prop_fund": "1",
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
                "appropriation": 10000000,
                "abc": 0,
                "status_id": {
                  "data": null
                },
                "remarks": null,
                "entry_type": "IMPLEMENTED",
                "bid_date": "2022-01-26",
                "year": 2021,
                "abc_published": null,
                "prop_takers": null,
                "prop_infra": null,
                "time_extension": null
              }
            }), db_connection).await,
    convert_projects_into_active_model_v2(&json!({
              "id": "182",
              "attributes": {
                "name": "Improvement of Motorpool Building\n",
                "code": "RF21 - 8000 - 3B - 1",
                "location": "[]",
                "cost": 1932993.58,
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
                "prop_fund": "1",
                "prop_type": "26",
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
                "start": null,
                "target": null,
                "office": "Engineer's Office",
                "assigned": null,
                "appropriation": 2000000,
                "abc": 0,
                "status_id": {
                  "data": null
                },
                "remarks": null,
                "entry_type": "IMPLEMENTED",
                "bid_date": "2021-06-30",
                "year": 2021,
                "abc_published": null,
                "prop_takers": null,
                "prop_infra": null,
                "time_extension": null
              }
            }), db_connection).await,
    convert_projects_into_active_model_v2(&json!({
              "id": "183",
              "attributes": {
                "name": " Const/Impvt of Heavy Equipment Garage at Old Solid Waste, Bagong Silang",
                "code": "RF21-8000-3B-2",
                "location": "[\"1\"]",
                "cost": 3851768.16,
                "contractor": {
                  "data": {
                    "id": "19",
                    "attributes": {
                      "name": "Jenrich Construction",
                      "street_purok": null
                    }
                  }
                },
                "duration": 180,
                "adjusted": null,
                "total": null,
                "weight": null,
                "prop_sdg": "[]",
                "prop_fund": "1",
                "prop_type": "26",
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
                "start": "2023-10-09",
                "target": "2024-04-06",
                "office": "Engineer's Office",
                "assigned": null,
                "appropriation": 4000000,
                "abc": 3851768.16,
                "status_id": {
                  "data": null
                },
                "remarks": "\"*100% physically completed based on approved technical plan & specifications\n* Adjusted Target Date of Completion: April 22, 2024 as per Variation Order No.1\n* Actual Completion Date: April 22, 2024\"\n",
                "entry_type": "IMPLEMENTED",
                "bid_date": "2021-09-21",
                "year": 2021,
                "abc_published": null,
                "prop_takers": null,
                "prop_infra": null,
                "time_extension": null
              }
            }), db_connection).await,
    convert_projects_into_active_model_v2(&json!({
              "id": "184",
              "attributes": {
                "name": " Improvement of City Hall Complex/Building/Drainage System",
                "code": "RF21 - 8000 - 3B - 3",
                "location": "[]",
                "cost": 1904658.27,
                "contractor": {
                  "data": {
                    "id": "16",
                    "attributes": {
                      "name": "AJT Construction",
                      "street_purok": null
                    }
                  }
                },
                "duration": null,
                "adjusted": null,
                "total": null,
                "weight": null,
                "prop_sdg": "[]",
                "prop_fund": "1",
                "prop_type": "26",
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
                "start": null,
                "target": null,
                "office": "Engineer's Office",
                "assigned": null,
                "appropriation": 2000000,
                "abc": 0,
                "status_id": {
                  "data": null
                },
                "remarks": null,
                "entry_type": "IMPLEMENTED",
                "bid_date": "2022-01-26",
                "year": 2021,
                "abc_published": null,
                "prop_takers": null,
                "prop_infra": null,
                "time_extension": null
              }
            }), db_connection).await,
    convert_projects_into_active_model_v2(&json!({
              "id": "185",
              "attributes": {
                "name": "Construction of New City Hall Building at Formerly Philpost & DENR Location",
                "code": "RF21 - 8000 - 3B - 4",
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
                "prop_fund": "1",
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
                "appropriation": 2000000,
                "abc": 0,
                "status_id": {
                  "data": null
                },
                "remarks": null,
                "entry_type": "IMPLEMENTED",
                "bid_date": null,
                "year": 2021,
                "abc_published": null,
                "prop_takers": null,
                "prop_infra": null,
                "time_extension": null
              }
            }), db_connection).await,
    convert_projects_into_active_model_v2(&json!({
              "id": "186",
              "attributes": {
                "name": "Construction of City Government Warehouse Building Phase 1 (RF21-8000-3B-5 , p1,029,500.00) and Construction of Structures  at Bagong Silang Dumpsite (RF21-8000-3B-5, P320T)",
                "code": "\"RF21-8000-3B-5/ 4SAIP22-8000R-B-1b11\"",
                "location": "[\"1\"]",
                "cost": 1320798.48,
                "contractor": {
                  "data": {
                    "id": "20",
                    "attributes": {
                      "name": "Maqueserg Construction",
                      "street_purok": null
                    }
                  }
                },
                "duration": 152,
                "adjusted": null,
                "total": null,
                "weight": null,
                "prop_sdg": "[]",
                "prop_fund": "1",
                "prop_type": "26",
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
                "start": "2024-06-19",
                "target": "2024-11-18",
                "office": "Engineer's Office",
                "assigned": null,
                "appropriation": 1349500,
                "abc": 1322552.33,
                "status_id": {
                  "data": null
                },
                "remarks": "\"* 100% physically completed based on approved technical plans & specifications\n* Adjusted Target date of Completion:  April 25, 2024\n* Actual Completion Date: 2/22/2024\"\n",
                "entry_type": "IMPLEMENTED",
                "bid_date": null,
                "year": 2021,
                "abc_published": null,
                "prop_takers": null,
                "prop_infra": null,
                "time_extension": null
              }
            }), db_connection).await,
    convert_projects_into_active_model_v2(&json!({
              "id": "187",
              "attributes": {
                "name": "Construction of Sidewalk/Drainage Along North Diversion Road",
                "code": "RF21 - 8000 - 3C - 1",
                "location": "[]",
                "cost": 1937235.33,
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
                "prop_fund": "1",
                "prop_type": "7",
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
                "appropriation": 2000000,
                "abc": 0,
                "status_id": {
                  "data": null
                },
                "remarks": null,
                "entry_type": "IMPLEMENTED",
                "bid_date": "2021-04-10",
                "year": 2021,
                "abc_published": null,
                "prop_takers": null,
                "prop_infra": null,
                "time_extension": null
              }
            }), db_connection).await,
    convert_projects_into_active_model_v2(&json!({
              "id": "188",
              "attributes": {
                "name": "Construction of Sidewalk/Drainage Along National Highway from Balangasan to Balintawak\n",
                "code": "RF21 - 8000 - 3C - 2",
                "location": "[]",
                "cost": 3877989.6,
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
                "prop_fund": "1",
                "prop_type": "7",
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
                "appropriation": 4000000,
                "abc": 0,
                "status_id": {
                  "data": null
                },
                "remarks": null,
                "entry_type": "IMPLEMENTED",
                "bid_date": "2021-08-31",
                "year": 2021,
                "abc_published": null,
                "prop_takers": null,
                "prop_infra": null,
                "time_extension": null
              }
            }), db_connection).await,
    convert_projects_into_active_model_v2(&json!({
              "id": "189",
              "attributes": {
                "name": "Construction of Sidewalk/Drainage Along National Highway from Tiguma - Tawagan Sur\n",
                "code": "RF21 - 8000 - 3C - 3",
                "location": "[]",
                "cost": 3849433.09,
                "contractor": {
                  "data": {
                    "id": "23",
                    "attributes": {
                      "name": "KS Agad Construction",
                      "street_purok": null
                    }
                  }
                },
                "duration": null,
                "adjusted": null,
                "total": null,
                "weight": null,
                "prop_sdg": "[]",
                "prop_fund": "1",
                "prop_type": "7",
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
                "appropriation": 4000000,
                "abc": 0,
                "status_id": {
                  "data": null
                },
                "remarks": null,
                "entry_type": "IMPLEMENTED",
                "bid_date": "2021-12-27",
                "year": 2021,
                "abc_published": null,
                "prop_takers": null,
                "prop_infra": null,
                "time_extension": null
              }
            }), db_connection).await,
            convert_projects_into_active_model_v2(&json!({
              "id": "190",
              "attributes": {
                "name": "Along Sabado St. (Jct. Duterte St - Sagun St.) Eastern Side\n",
                "code": "RF21- 8000 - 3c - 4a",
                "location": "[]",
                "cost": 1161960.02,
                "contractor": {
                  "data": {
                    "id": "24",
                    "attributes": {
                      "name": "MG Construction",
                      "street_purok": null
                    }
                  }
                },
                "duration": null,
                "adjusted": null,
                "total": null,
                "weight": null,
                "prop_sdg": "[]",
                "prop_fund": "1",
                "prop_type": "7",
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
                "appropriation": 1200000,
                "abc": 0,
                "status_id": {
                  "data": null
                },
                "remarks": null,
                "entry_type": "IMPLEMENTED",
                "bid_date": "2021-11-08",
                "year": 2021,
                "abc_published": null,
                "prop_takers": null,
                "prop_infra": null,
                "time_extension": null
              }
            }), db_connection).await,
    convert_projects_into_active_model_v2(&json!({
              "id": "191",
              "attributes": {
                "name": "Along Sabado St. (Jct. Duterte St - Sagun St.)Western Side\n",
                "code": "RF21- 8000 - 3c - 4b",
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
                "prop_fund": "1",
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
                "appropriation": 1200000,
                "abc": 0,
                "status_id": {
                  "data": null
                },
                "remarks": null,
                "entry_type": "IMPLEMENTED",
                "bid_date": null,
                "year": 2021,
                "abc_published": null,
                "prop_takers": null,
                "prop_infra": null,
                "time_extension": null
              }
            }), db_connection).await,
    convert_projects_into_active_model_v2(&json!({
              "id": "192",
              "attributes": {
                "name": "Along Dablo St. (FS Pajares St - Macaumbang St) Northern Side\n",
                "code": "RF21- 8000 - 3c - 4c",
                "location": "[]",
                "cost": 4055778.11,
                "contractor": {
                  "data": {
                    "id": "13",
                    "attributes": {
                      "name": "Glenson Construction",
                      "street_purok": null
                    }
                  }
                },
                "duration": null,
                "adjusted": null,
                "total": null,
                "weight": null,
                "prop_sdg": "[]",
                "prop_fund": "1",
                "prop_type": "7",
                "prop_category": "1",
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
                "appropriation": 4200000,
                "abc": 0,
                "status_id": {
                  "data": null
                },
                "remarks": "95% PHYSICAL ACCOMPLISHMENT, temporarily suspended due to unressolve  road right of way problem, encroachment with private establishment ( KUSINA RESTO)",
                "entry_type": "IMPLEMENTED",
                "bid_date": "2021-06-30",
                "year": 2021,
                "abc_published": null,
                "prop_takers": null,
                "prop_infra": null,
                "time_extension": null
              }
            }), db_connection).await,
    convert_projects_into_active_model_v2(&json!({
              "id": "193",
              "attributes": {
                "name": "Along Dablo St (Macaumbang St - Sabellano St) Northern Side\n",
                "code": "RF21- 8000 - 3c - 4d",
                "location": "[]",
                "cost": 4055874.19,
                "contractor": {
                  "data": {
                    "id": "25",
                    "attributes": {
                      "name": "Five Star Builders",
                      "street_purok": null
                    }
                  }
                },
                "duration": null,
                "adjusted": null,
                "total": null,
                "weight": null,
                "prop_sdg": "[]",
                "prop_fund": "1",
                "prop_type": "7",
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
                "appropriation": 4200000,
                "abc": 0,
                "status_id": {
                  "data": null
                },
                "remarks": null,
                "entry_type": "IMPLEMENTED",
                "bid_date": "2021-11-08",
                "year": 2021,
                "abc_published": null,
                "prop_takers": null,
                "prop_infra": null,
                "time_extension": null
              }
            }), db_connection).await,
    convert_projects_into_active_model_v2(&json!({
              "id": "194",
              "attributes": {
                "name": "CONSTRUCTION OF SIDEWALK WITH UNDERGROUND DRAINAGE  Along Duterte St. (Urro St -  Sanson St) Western Side",
                "code": "RF21-8000-3c-4e",
                "location": "[\"7\"]",
                "cost": 2133293.3,
                "contractor": {
                  "data": {
                    "id": "10",
                    "attributes": {
                      "name": "RSQ Construction",
                      "street_purok": null
                    }
                  }
                },
                "duration": 90,
                "adjusted": null,
                "total": null,
                "weight": null,
                "prop_sdg": "[]",
                "prop_fund": "1",
                "prop_type": "7",
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
                "start": "2021-10-21",
                "target": "2022-01-19",
                "office": "Engineer's Office",
                "assigned": null,
                "appropriation": 2200000,
                "abc": 0,
                "status_id": {
                  "data": null
                },
                "remarks": "\"* 100% physically completed as per approved technical specifications\n* w/ time extension  as per approved SO No.1  due to road right of way problem and  VO No. 1\"\n",
                "entry_type": "IMPLEMENTED",
                "bid_date": "2021-06-30",
                "year": 2021,
                "abc_published": null,
                "prop_takers": null,
                "prop_infra": null,
                "time_extension": null
              }
            }), db_connection).await,
    convert_projects_into_active_model_v2(&json!({
              "id": "195",
              "attributes": {
                "name": "Construction of Breakwater at Boulevard\n",
                "code": "RF21 - 8000 - 3C - 5",
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
                "prop_fund": "1",
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
                "appropriation": 10000000,
                "abc": 0,
                "status_id": {
                  "data": null
                },
                "remarks": null,
                "entry_type": "IMPLEMENTED",
                "bid_date": null,
                "year": 2021,
                "abc_published": null,
                "prop_takers": null,
                "prop_infra": null,
                "time_extension": null
              }
            }), db_connection).await,
    convert_projects_into_active_model_v2(&json!({
              "id": "196",
              "attributes": {
                "name": "Alignment of Boulevard/Embankment/Concreting (Continuation of Boulevard Beside Fishport)",
                "code": "RF21-8000-3C-6",
                "location": "[\"13\"]",
                "cost": 5852121.21,
                "contractor": {
                  "data": {
                    "id": "1",
                    "attributes": {
                      "name": "CTG Construction",
                      "street_purok": "Purok 1"
                    }
                  }
                },
                "duration": 120,
                "adjusted": null,
                "total": null,
                "weight": null,
                "prop_sdg": "[]",
                "prop_fund": "1",
                "prop_type": "11",
                "prop_category": "1",
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
                "start": "2024-01-29",
                "target": "2024-05-28",
                "office": "Engineer's Office",
                "assigned": null,
                "appropriation": 6000000,
                "abc": 5853322.61,
                "status_id": {
                  "data": null
                },
                "remarks": "\"* On-going  implementation of the project\n* with time extension as per approved suspension order\n* Adjusted Target date of Completion:July 29, 2024\"\n",
                "entry_type": "IMPLEMENTED",
                "bid_date": "2021-12-06",
                "year": 2021,
                "abc_published": null,
                "prop_takers": null,
                "prop_infra": null,
                "time_extension": null
              }
            }), db_connection).await,
    convert_projects_into_active_model_v2(&json!({
              "id": "197",
              "attributes": {
                "name": "Along Sagun St. (Jct Datoc St - Sagun Bridge) Northern Side\n",
                "code": "DRRMF21 - 1000 - 2 -a1 ",
                "location": "[]",
                "cost": 1920555.84,
                "contractor": {
                  "data": {
                    "id": "26",
                    "attributes": {
                      "name": "ZMS Construction",
                      "street_purok": null
                    }
                  }
                },
                "duration": null,
                "adjusted": null,
                "total": null,
                "weight": null,
                "prop_sdg": "[]",
                "prop_fund": "3",
                "prop_type": "7",
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
                "appropriation": 2000000,
                "abc": 0,
                "status_id": {
                  "data": null
                },
                "remarks": null,
                "entry_type": "IMPLEMENTED",
                "bid_date": "2021-06-08",
                "year": 2021,
                "abc_published": null,
                "prop_takers": null,
                "prop_infra": null,
                "time_extension": null
              }
            }), db_connection).await,
    convert_projects_into_active_model_v2(&json!({
              "id": "198",
              "attributes": {
                "name": "Along Sagun St (Jct Aquino St - Datoc St) Northern Side\n",
                "code": "DRRMF21 - 1000 - 2 -a2 ",
                "location": "[]",
                "cost": 1155565.39,
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
                "prop_fund": "3",
                "prop_type": "7",
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
                "appropriation": 1200000,
                "abc": 0,
                "status_id": {
                  "data": null
                },
                "remarks": null,
                "entry_type": "IMPLEMENTED",
                "bid_date": "2021-04-20",
                "year": 2021,
                "abc_published": null,
                "prop_takers": null,
                "prop_infra": null,
                "time_extension": null
              }
            }), db_connection).await,
    convert_projects_into_active_model_v2(&json!({
              "id": "199",
              "attributes": {
                "name": "Along Sagun St. (Jct Alano St - Roxas St) Northern Side\n",
                "code": "DRRMF21 - 1000 - 2 -a3 ",
                "location": "[]",
                "cost": 2309213.74,
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
                "prop_fund": "3",
                "prop_type": "7",
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
                "appropriation": 2400000,
                "abc": 0,
                "status_id": {
                  "data": null
                },
                "remarks": null,
                "entry_type": "IMPLEMENTED",
                "bid_date": "2021-09-29",
                "year": 2021,
                "abc_published": null,
                "prop_takers": null,
                "prop_infra": null,
                "time_extension": null
              }
            }), db_connection).await,
            convert_projects_into_active_model_v2(&json!({
              "id": "200",
              "attributes": {
                "name": "Along Sagun St. (Jct Roxas St - Bana St) Northern Side\n",
                "code": "DRRMF21 - 1000 - 2 -a4 ",
                "location": "[]",
                "cost": 2295241.03,
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
                "prop_fund": "3",
                "prop_type": "7",
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
                "appropriation": 2400000,
                "abc": 0,
                "status_id": {
                  "data": null
                },
                "remarks": null,
                "entry_type": "IMPLEMENTED",
                "bid_date": "2021-12-06",
                "year": 2021,
                "abc_published": null,
                "prop_takers": null,
                "prop_infra": null,
                "time_extension": null
              }
            }), db_connection).await,
    convert_projects_into_active_model_v2(&json!({
              "id": "201",
              "attributes": {
                "name": "Along Sagun St (Jct FS Pajares Ave - Alano St) Southern side\n",
                "code": "DRRMF21 - 1000 - 2 -a5",
                "location": "[]",
                "cost": 2314962.2,
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
                "prop_fund": "3",
                "prop_type": "7",
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
                "appropriation": 2400000,
                "abc": 0,
                "status_id": {
                  "data": null
                },
                "remarks": null,
                "entry_type": "IMPLEMENTED",
                "bid_date": "2021-09-29",
                "year": 2021,
                "abc_published": null,
                "prop_takers": null,
                "prop_infra": null,
                "time_extension": null
              }
            }), db_connection).await,
    convert_projects_into_active_model_v2(&json!({
              "id": "202",
              "attributes": {
                "name": " Along Sagun St. (Jct Alano - Roxas St) Southern Side",
                "code": "DRRMF21 - 1000 - 2 -a6",
                "location": "[]",
                "cost": 2319071.22,
                "contractor": {
                  "data": {
                    "id": "27",
                    "attributes": {
                      "name": "JZL Builders",
                      "street_purok": null
                    }
                  }
                },
                "duration": null,
                "adjusted": null,
                "total": null,
                "weight": null,
                "prop_sdg": "[]",
                "prop_fund": "3",
                "prop_type": "7",
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
                "appropriation": 2400000,
                "abc": 0,
                "status_id": {
                  "data": null
                },
                "remarks": null,
                "entry_type": "IMPLEMENTED",
                "bid_date": "2021-04-20",
                "year": 2021,
                "abc_published": null,
                "prop_takers": null,
                "prop_infra": null,
                "time_extension": null
              }
            }), db_connection).await,
    convert_projects_into_active_model_v2(&json!({
              "id": "203",
              "attributes": {
                "name": "Construction of Sidewalk/Drainage Along Ariosa St (jct Roxas St - Bana St) Southern Side",
                "code": "DRRMF21-1000-2-b7",
                "location": "[]",
                "cost": 2320402.54,
                "contractor": {
                  "data": {
                    "id": "15",
                    "attributes": {
                      "name": "Aradel Construction",
                      "street_purok": null
                    }
                  }
                },
                "duration": 130,
                "adjusted": null,
                "total": null,
                "weight": null,
                "prop_sdg": "[]",
                "prop_fund": "3",
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
                "start": "2023-02-23",
                "target": "2023-07-03",
                "office": "Engineer's Office",
                "assigned": null,
                "appropriation": 2400000,
                "abc": 0,
                "status_id": {
                  "data": null
                },
                "remarks": "100% physically completed based on approved technical plans & specifications\n\nActual Date Completed: 7/2/2023\n",
                "entry_type": "IMPLEMENTED",
                "bid_date": "2023-01-17",
                "year": 2021,
                "abc_published": null,
                "prop_takers": "EMELYE ARANAS 0999991309",
                "prop_infra": null,
                "time_extension": null
              }
            }), db_connection).await,
    convert_projects_into_active_model_v2(&json!({
              "id": "204",
              "attributes": {
                "name": "CONSTRUCTION OFSIDEWALK AND UNDERGROUND DRAINAGE ALONG RT LIM STREET (JCT URRO ST - SAGUN ST)ESTERN SIDE -",
                "code": "DRRMF21-1000-2-a8",
                "location": "[]",
                "cost": 1739993.58,
                "contractor": {
                  "data": {
                    "id": "28",
                    "attributes": {
                      "name": "NRMM Construction",
                      "street_purok": null
                    }
                  }
                },
                "duration": 90,
                "adjusted": null,
                "total": null,
                "weight": null,
                "prop_sdg": "[]",
                "prop_fund": "3",
                "prop_type": "7",
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
                "start": "2021-09-20",
                "target": "2021-12-19",
                "office": "Engineer's Office",
                "assigned": null,
                "appropriation": 1800000,
                "abc": 0,
                "status_id": {
                  "data": null
                },
                "remarks": "100% physically completed based on approved technical plans & specifications\n\nActual Completion Date: 4/28/2022",
                "entry_type": "IMPLEMENTED",
                "bid_date": "2021-06-30",
                "year": 2021,
                "abc_published": null,
                "prop_takers": null,
                "prop_infra": null,
                "time_extension": null
              }
            }), db_connection).await,
    convert_projects_into_active_model_v2(&json!({
              "id": "205",
              "attributes": {
                "name": "Along Datoc St. (Jct Salazar St - Urro St) Western Side\n",
                "code": "DRRMF21 - 1000 - 2 -a9",
                "location": "[]",
                "cost": 1145423,
                "contractor": {
                  "data": {
                    "id": "28",
                    "attributes": {
                      "name": "NRMM Construction",
                      "street_purok": null
                    }
                  }
                },
                "duration": null,
                "adjusted": null,
                "total": null,
                "weight": null,
                "prop_sdg": "[]",
                "prop_fund": "3",
                "prop_type": "7",
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
                "appropriation": 1200000,
                "abc": 0,
                "status_id": {
                  "data": null
                },
                "remarks": "",
                "entry_type": "IMPLEMENTED",
                "bid_date": "2021-05-10",
                "year": 2021,
                "abc_published": null,
                "prop_takers": null,
                "prop_infra": null,
                "time_extension": null
              }
            }), db_connection).await,
    convert_projects_into_active_model_v2(&json!({
              "id": "206",
              "attributes": {
                "name": "Along Aquino St. (Jct Urro St - Sagun St) Western Side\n",
                "code": "DRRMF21 - 1000 - 2 -a10",
                "location": "[]",
                "cost": 966987.14,
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
                "prop_fund": "3",
                "prop_type": "7",
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
                "appropriation": 1000000,
                "abc": 0,
                "status_id": {
                  "data": null
                },
                "remarks": null,
                "entry_type": "IMPLEMENTED",
                "bid_date": "2021-07-20",
                "year": 2021,
                "abc_published": null,
                "prop_takers": null,
                "prop_infra": null,
                "time_extension": null
              }
            }), db_connection).await,
    convert_projects_into_active_model_v2(&json!({
              "id": "207",
              "attributes": {
                "name": "Along FS Pajares Ave. (Jct Broca St - Urro St) Western Side\n",
                "code": "DRRMF21 - 1000 - 2 -a11",
                "location": "[]",
                "cost": 1525498.38,
                "contractor": {
                  "data": {
                    "id": "13",
                    "attributes": {
                      "name": "Glenson Construction",
                      "street_purok": null
                    }
                  }
                },
                "duration": null,
                "adjusted": null,
                "total": null,
                "weight": null,
                "prop_sdg": "[]",
                "prop_fund": "3",
                "prop_type": "7",
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
                "appropriation": 1600000,
                "abc": 0,
                "status_id": {
                  "data": null
                },
                "remarks": " Conflicts in the RRW \n",
                "entry_type": "IMPLEMENTED",
                "bid_date": "2021-04-26",
                "year": 2021,
                "abc_published": null,
                "prop_takers": null,
                "prop_infra": null,
                "time_extension": null
              }
            }), db_connection).await,
    convert_projects_into_active_model_v2(&json!({
              "id": "208",
              "attributes": {
                "name": "Along Mercedes St (Jct Aquino St - Fs Pajares Ave) Northern Side\n",
                "code": "DRRMF21 - 1000 - 2 -a13",
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
                "prop_fund": "3",
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
                "appropriation": 1200000,
                "abc": 0,
                "status_id": {
                  "data": null
                },
                "remarks": null,
                "entry_type": "IMPLEMENTED",
                "bid_date": null,
                "year": 2021,
                "abc_published": null,
                "prop_takers": null,
                "prop_infra": null,
                "time_extension": null
              }
            }), db_connection).await,
    convert_projects_into_active_model_v2(&json!({
              "id": "209",
              "attributes": {
                "name": "Along Urro St (Jct Alano St - Roxas St) Northern Side\n",
                "code": "DRRMF21 - 1000 - 2 -a14",
                "location": "[]",
                "cost": 2300471.46,
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
                "prop_fund": "3",
                "prop_type": "7",
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
                "appropriation": 2400000,
                "abc": 0,
                "status_id": {
                  "data": null
                },
                "remarks": null,
                "entry_type": "IMPLEMENTED",
                "bid_date": "2021-12-06",
                "year": 2021,
                "abc_published": null,
                "prop_takers": null,
                "prop_infra": null,
                "time_extension": null
              }
            }), db_connection).await,
            convert_projects_into_active_model_v2(&json!({
            "id": "210",
            "attributes": {
                "name": "Along Duterte St. (Jct Sagun St - Sanson St) Eastern Side\n",
                "code": "DRRMF21 - 1000 - 2 -a15",
                "location": "[]",
                "cost": 960997.38,
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
                "prop_fund": "3",
                "prop_type": "7",
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
                "appropriation": 1000000,
                "abc": 0,
                "status_id": {
                "data": null
                },
                "remarks": null,
                "entry_type": "IMPLEMENTED",
                "bid_date": "2021-04-12",
                "year": 2021,
                "abc_published": null,
                "prop_takers": null,
                "prop_infra": null,
                "time_extension": null
            }
            }), db_connection).await,
            convert_projects_into_active_model_v2(&json!({
            "id": "211",
            "attributes": {
                "name": "Along FS Pajares Ave\' (Jct Sagun St - Sanson St) Eastern Side\n",
                "code": "DRRMF21 - 1000 - 2 -a16",
                "location": "[]",
                "cost": 1157963.92,
                "contractor": {
                "data": {
                    "id": "9",
                    "attributes": {
                    "name": "Johairy Builders",
                    "street_purok": null
                    }
                }
                },
                "duration": null,
                "adjusted": null,
                "total": null,
                "weight": null,
                "prop_sdg": "[]",
                "prop_fund": "3",
                "prop_type": "7",
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
                "appropriation": 1200000,
                "abc": 0,
                "status_id": {
                "data": null
                },
                "remarks": null,
                "entry_type": "IMPLEMENTED",
                "bid_date": "2022-08-15",
                "year": 2021,
                "abc_published": null,
                "prop_takers": null,
                "prop_infra": null,
                "time_extension": null
            }
            }), db_connection).await,
            convert_projects_into_active_model_v2(&json!({
            "id": "212",
            "attributes": {
                "name": "Along Alano St (Jct Sagun St - Sanson St) Western Side\n",
                "code": "DRRMF21 - 1000 - 2 -a17",
                "location": "[]",
                "cost": 1152979.48,
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
                "prop_fund": "3",
                "prop_type": "7",
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
                "appropriation": 1200000,
                "abc": 0,
                "status_id": {
                "data": null
                },
                "remarks": null,
                "entry_type": "IMPLEMENTED",
                "bid_date": "2021-05-10",
                "year": 2021,
                "abc_published": null,
                "prop_takers": null,
                "prop_infra": null,
                "time_extension": null
            }
            }), db_connection).await,
            convert_projects_into_active_model_v2(&json!({
            "id": "213",
            "attributes": {
                "name": "Along FS Pajares Ave( Jamisola St - Pulmones St) Western Side\n",
                "code": "DRRMF21 - 1000 - 2 -a18",
                "location": "[]",
                "cost": 1152978.92,
                "contractor": {
                "data": {
                    "id": "29",
                    "attributes": {
                    "name": "Genetian Builders",
                    "street_purok": null
                    }
                }
                },
                "duration": null,
                "adjusted": null,
                "total": null,
                "weight": null,
                "prop_sdg": "[]",
                "prop_fund": "3",
                "prop_type": "7",
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
                "appropriation": 1200000,
                "abc": 0,
                "status_id": {
                "data": null
                },
                "remarks": null,
                "entry_type": "IMPLEMENTED",
                "bid_date": "2021-06-30",
                "year": 2021,
                "abc_published": null,
                "prop_takers": null,
                "prop_infra": null,
                "time_extension": null
            }
            }), db_connection).await,
            convert_projects_into_active_model_v2(&json!({
            "id": "214",
            "attributes": {
                "name": "Along Duterte St (Jct Jamisola St - Dablo St) Western Side\n",
                "code": "DRRMF21 - 1000 - 2 -a19",
                "location": "[]",
                "cost": 3325879.2,
                "contractor": {
                "data": {
                    "id": "29",
                    "attributes": {
                    "name": "Genetian Builders",
                    "street_purok": null
                    }
                }
                },
                "duration": null,
                "adjusted": null,
                "total": null,
                "weight": null,
                "prop_sdg": "[]",
                "prop_fund": "3",
                "prop_type": "7",
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
                "appropriation": 3500000,
                "abc": 0,
                "status_id": {
                "data": null
                },
                "remarks": " Road Right of Way Problem \n",
                "entry_type": "IMPLEMENTED",
                "bid_date": "2021-04-20",
                "year": 2021,
                "abc_published": null,
                "prop_takers": null,
                "prop_infra": null,
                "time_extension": null
            }
            }), db_connection).await,
            convert_projects_into_active_model_v2(&json!({
            "id": "215",
            "attributes": {
                "name": "Along Purok Lawis (Jct Pulmones - Dablo St)\n",
                "code": "DRRMF21 - 1000 - 2 -a20",
                "location": "[]",
                "cost": 1929942.79,
                "contractor": {
                "data": {
                    "id": "5",
                    "attributes": {
                    "name": "MOLROW CONSTRUCTION",
                    "street_purok": null
                    }
                }
                },
                "duration": null,
                "adjusted": null,
                "total": null,
                "weight": null,
                "prop_sdg": "[]",
                "prop_fund": "3",
                "prop_type": "7",
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
                "appropriation": 2000000,
                "abc": 0,
                "status_id": {
                "data": null
                },
                "remarks": null,
                "entry_type": "IMPLEMENTED",
                "bid_date": "2021-09-29",
                "year": 2021,
                "abc_published": null,
                "prop_takers": null,
                "prop_infra": null,
                "time_extension": null
            }
            }), db_connection).await,
            convert_projects_into_active_model_v2(&json!({
            "id": "216",
            "attributes": {
                "name": "Along Jamisola St (Jct Duterte St - Manaya St) Southen Side\n",
                "code": "DRRMF21 - 1000 - 2 -a21",
                "location": "[]",
                "cost": 1928206.15,
                "contractor": {
                "data": {
                    "id": "29",
                    "attributes": {
                    "name": "Genetian Builders",
                    "street_purok": null
                    }
                }
                },
                "duration": null,
                "adjusted": null,
                "total": null,
                "weight": null,
                "prop_sdg": "[]",
                "prop_fund": "3",
                "prop_type": "7",
                "prop_category": "1",
                "prop_sector": "[]",
                "prop_assign": "2",
                "accom_total": 70.01,
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
                "remarks": "On-going but non-moving",
                "entry_type": "IMPLEMENTED",
                "bid_date": "2021-05-10",
                "year": 2021,
                "abc_published": null,
                "prop_takers": null,
                "prop_infra": null,
                "time_extension": null
            }
            }), db_connection).await,
            convert_projects_into_active_model_v2(&json!({
            "id": "217",
            "attributes": {
                "name": "Along Jamisola St (Jct Zulueta St - Manaya St) Northern Side\n",
                "code": "DRRMF21 - 1000 - 2 -a22",
                "location": "[]",
                "cost": 1158399.87,
                "contractor": {
                "data": {
                    "id": "29",
                    "attributes": {
                    "name": "Genetian Builders",
                    "street_purok": null
                    }
                }
                },
                "duration": null,
                "adjusted": null,
                "total": null,
                "weight": null,
                "prop_sdg": "[]",
                "prop_fund": "3",
                "prop_type": "7",
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
                "appropriation": 1200000,
                "abc": 0,
                "status_id": {
                "data": null
                },
                "remarks": null,
                "entry_type": "IMPLEMENTED",
                "bid_date": "2021-05-10",
                "year": 2021,
                "abc_published": null,
                "prop_takers": null,
                "prop_infra": null,
                "time_extension": null
            }
            }), db_connection).await,
            convert_projects_into_active_model_v2(&json!({
            "id": "218",
            "attributes": {
                "name": "Along Alano St (Jct Pulmones St - Dablo St) Eastern Side\n",
                "code": "DRRMF21 - 1000 - 2 -a23",
                "location": "[]",
                "cost": 1928922.27,
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
                "prop_fund": "3",
                "prop_type": "7",
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
                "appropriation": 2000000,
                "abc": 0,
                "status_id": {
                "data": null
                },
                "remarks": "\n Road Right of Way Problem \n",
                "entry_type": "IMPLEMENTED",
                "bid_date": "2021-04-26",
                "year": 2021,
                "abc_published": null,
                "prop_takers": null,
                "prop_infra": null,
                "time_extension": null
            }
            }), db_connection).await,
            convert_projects_into_active_model_v2(&json!({
            "id": "219",
            "attributes": {
                "name": "Along FS Pajares Ave (Jct Jamisola St - Pulmones St) Western Side\n",
                "code": "DRRMF21 - 1000 - 2 -a24",
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
                "prop_fund": "3",
                "prop_type": "7",
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
                "appropriation": 1200000,
                "abc": 0,
                "status_id": {
                "data": null
                },
                "remarks": null,
                "entry_type": "IMPLEMENTED",
                "bid_date": null,
                "year": 2021,
                "abc_published": null,
                "prop_takers": null,
                "prop_infra": null,
                "time_extension": null
            }
            }), db_connection).await,
            convert_projects_into_active_model_v2(&json!({
            "id": "220",
            "attributes": {
                "name": "Along D. Macaumbang St (Jct R Magsaysay Ave - Dablo St) Eastern Side\n",
                "code": "DRRMF21 - 1000 - 2 -a25",
                "location": "[]",
                "cost": 1936211.25,
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
                "prop_fund": "3",
                "prop_type": "7",
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
                "appropriation": 2000000,
                "abc": 0,
                "status_id": {
                "data": null
                },
                "remarks": null,
                "entry_type": "IMPLEMENTED",
                "bid_date": "2021-04-26",
                "year": 2021,
                "abc_published": null,
                "prop_takers": null,
                "prop_infra": null,
                "time_extension": null
            }
            }), db_connection).await,
convert_projects_into_active_model_v2(&json!({
            "id": "221",
            "attributes": {
                "name": "Concreting of Tiguma - White Beach Barangay Road",
                "code": "DF21-8000-B1a",
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
                "prop_fund": "2",
                "prop_type": "",
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
                "appropriation": 2200000,
                "abc": 0,
                "status_id": {
                "data": null
                },
                "remarks": null,
                "entry_type": "IMPLEMENTED",
                "bid_date": null,
                "year": 2021,
                "abc_published": null,
                "prop_takers": null,
                "prop_infra": null,
                "time_extension": null
            }
            }), db_connection).await,
convert_projects_into_active_model_v2(&json!({
            "id": "222",
            "attributes": {
                "name": "Rehabilitation/Concreting of Road Leading to Manga Falls",
                "code": "DF21-8000-B1b",
                "location": "[\"45\"]",
                "cost": 2894983.37,
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
                "prop_fund": "2",
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
                "target": "2023-08-18",
                "office": "Engineer's Office",
                "assigned": null,
                "appropriation": 3000000,
                "abc": 2896183.11,
                "status_id": {
                "data": null
                },
                "remarks": "100% physically completed based on approved technical plans & specifications\n",
                "entry_type": "IMPLEMENTED",
                "bid_date": null,
                "year": 2021,
                "abc_published": null,
                "prop_takers": "LANCE CO",
                "prop_infra": null,
                "time_extension": null
            }
            }), db_connection).await,
convert_projects_into_active_model_v2(&json!({
            "id": "223",
            "attributes": {
                "name": "Along New Tudela St. (Jct R. Magsaysay Ave - Dablo St) Eastern Side\n",
                "code": "DF21-8000-B2-a",
                "location": "[]",
                "cost": 2108988.38,
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
                "prop_fund": "2",
                "prop_type": "7",
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
                "appropriation": 2200000,
                "abc": 0,
                "status_id": {
                "data": null
                },
                "remarks": null,
                "entry_type": "IMPLEMENTED",
                "bid_date": "2021-06-30",
                "year": 2021,
                "abc_published": null,
                "prop_takers": null,
                "prop_infra": null,
                "time_extension": null
            }
            }), db_connection).await,
convert_projects_into_active_model_v2(&json!({
            "id": "224",
            "attributes": {
                "name": "Along Claro Recto St (Jct. R. Magsaysay Ave - Dablo St) Eastern Side\n",
                "code": "DF21-8000-B2-b",
                "location": "[]",
                "cost": 2424076.25,
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
                "prop_fund": "2",
                "prop_type": "7",
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
                "appropriation": 2500000,
                "abc": 0,
                "status_id": {
                "data": null
                },
                "remarks": null,
                "entry_type": "IMPLEMENTED",
                "bid_date": "2021-11-08",
                "year": 2021,
                "abc_published": null,
                "prop_takers": null,
                "prop_infra": null,
                "time_extension": null
            }
            }), db_connection).await,
convert_projects_into_active_model_v2(&json!({
            "id": "225",
            "attributes": {
                "name": "Construction of Sidewalk and Underground Drainage   Along Sabado St (Jct Hofilena St - sagun St) Eastern & Western Side AND  Const. of Sidewalk w/ Underground Drainage along Sabado St. (Jct. Sagun St.-Hofilena St.) Bothsides, Brgy. San Francisco  (DF21-80",
                "code": "DF21-8000-B2-c",
                "location": "[]",
                "cost": 2990294.94,
                "contractor": {
                "data": {
                    "id": "26",
                    "attributes": {
                    "name": "ZMS Construction",
                    "street_purok": null
                    }
                }
                },
                "duration": 0,
                "adjusted": null,
                "total": null,
                "weight": null,
                "prop_sdg": "[]",
                "prop_fund": "2",
                "prop_type": "7",
                "prop_category": "1",
                "prop_sector": "[]",
                "prop_assign": "2",
                "accom_total": 56,
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
                "start": "2023-12-14",
                "target": "2023-12-14",
                "office": "Engineer's Office",
                "assigned": null,
                "appropriation": 3000000,
                "abc": 0,
                "status_id": {
                "data": null
                },
                "remarks": "ON-GOING W/ 50% ACCOMPLISHMENT AS OF MARCH 8, 2024\n",
                "entry_type": "IMPLEMENTED",
                "bid_date": "2021-08-31",
                "year": 2021,
                "abc_published": null,
                "prop_takers": "BOLOTAOLO REVELO",
                "prop_infra": null,
                "time_extension": null
            }
            }), db_connection).await,
convert_projects_into_active_model_v2(&json!({
            "id": "226",
            "attributes": {
                "name": "Along Duterte St (Jct Urro St. - Sanson St) Eastern Side\n",
                "code": "DF21-8000-B2-d",
                "location": "[]",
                "cost": 1927703.33,
                "contractor": {
                "data": {
                    "id": "3",
                    "attributes": {
                    "name": "GAF TRADING & CONST",
                    "street_purok": null
                    }
                }
                },
                "duration": null,
                "adjusted": null,
                "total": null,
                "weight": null,
                "prop_sdg": "[]",
                "prop_fund": "2",
                "prop_type": "7",
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
                "appropriation": 2000000,
                "abc": 0,
                "status_id": {
                "data": null
                },
                "remarks": null,
                "entry_type": "IMPLEMENTED",
                "bid_date": "2021-07-14",
                "year": 2021,
                "abc_published": null,
                "prop_takers": null,
                "prop_infra": null,
                "time_extension": null
            }
            }), db_connection).await,
convert_projects_into_active_model_v2(&json!({
            "id": "227",
            "attributes": {
                "name": "Along Cabrera St (Jct. Alano St - Sabado St) Northern Side\n",
                "code": "DF21-8000-B2-e",
                "location": "[]",
                "cost": 1159978.98,
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
                "prop_fund": "2",
                "prop_type": "7",
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
                "appropriation": 1200000,
                "abc": 0,
                "status_id": {
                "data": null
                },
                "remarks": " on-going but non moving ",
                "entry_type": "IMPLEMENTED",
                "bid_date": "2021-04-20",
                "year": 2021,
                "abc_published": null,
                "prop_takers": null,
                "prop_infra": null,
                "time_extension": null
            }
            }), db_connection).await,
convert_projects_into_active_model_v2(&json!({
            "id": "228",
            "attributes": {
                "name": "Renovation of Fish Section Building (agora Market)\n",
                "code": "DF21 - 8000 - B3 - a",
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
                "prop_fund": "2",
                "prop_type": "26",
                "prop_category": "3",
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
                "appropriation": 10000000,
                "abc": 0,
                "status_id": {
                "data": null
                },
                "remarks": null,
                "entry_type": "IMPLEMENTED",
                "bid_date": null,
                "year": 2021,
                "abc_published": null,
                "prop_takers": null,
                "prop_infra": null,
                "time_extension": null
            }
            }), db_connection).await,
convert_projects_into_active_model_v2(&json!({
            "id": "229",
            "attributes": {
                "name": "Construction of HALAL Section Building ( Agora Market)\n",
                "code": "DF21 - 8000 - B3 - b",
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
                "prop_fund": "2",
                "prop_type": "26",
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
                "appropriation": 2000000,
                "abc": 0,
                "status_id": {
                "data": null
                },
                "remarks": null,
                "entry_type": "IMPLEMENTED",
                "bid_date": null,
                "year": 2021,
                "abc_published": null,
                "prop_takers": null,
                "prop_infra": null,
                "time_extension": null
            }
            }), db_connection).await,
            convert_projects_into_active_model_v2(&json!({
    "id": "230",
    "attributes": {
        "name": "Completion of Agora Perimeter Building (Back Portion of Agora Market)",
        "code": "DF21-8000-B3-c",
        "location": "[\"14\"]",
        "cost": 14477795.73,
        "contractor": {
            "data": {
                "id": "31",
                "attributes": {
                    "name": "OG Santos Construction",
                    "street_purok": null
                }
            }
        },
        "duration": 250,
        "adjusted": null,
        "total": null,
        "weight": null,
        "prop_sdg": "[]",
        "prop_fund": "2",
        "prop_type": "26",
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
        "start": "2023-03-07",
        "target": "2023-11-12",
        "office": "Engineer's Office",
        "assigned": null,
        "appropriation": 15000000,
        "abc": 14500907.96,
        "status_id": {
            "data": null
        },
        "remarks": "\"*100% physically completed based on approved technical plans & specifications\n* date Completed: March 18, 2024\"\n",
        "entry_type": "IMPLEMENTED",
        "bid_date": "2022-11-10",
        "year": 2021,
        "abc_published": null,
        "prop_takers": "Oliver Santos",
        "prop_infra": null,
        "time_extension": null
    }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
    "id": "231",
    "attributes": {
        "name": "Continuation of Construction of Covered Court (TilesInstallation on Stage)\n",
        "code": "DF21 - 8000 - B4 - a",
        "location": "[]",
        "cost": 3866150.15,
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
        "prop_fund": "2",
        "prop_type": "16",
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
        "start": null,
        "target": null,
        "office": "Engineer's Office",
        "assigned": null,
        "appropriation": 4000000,
        "abc": 0,
        "status_id": {
            "data": null
        },
        "remarks": null,
        "entry_type": "IMPLEMENTED",
        "bid_date": "2021-06-08",
        "year": 2021,
        "abc_published": null,
        "prop_takers": null,
        "prop_infra": null,
        "time_extension": null
    }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
    "id": "232",
    "attributes": {
        "name": " Improvement of Boulevard at San Pedro Including Beautification and Landscaping\n",
        "code": "DF21-8000-B4-b",
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
        "prop_fund": "2",
        "prop_type": "",
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
        "appropriation": 5000000,
        "abc": 0,
        "status_id": {
            "data": null
        },
        "remarks": null,
        "entry_type": "IMPLEMENTED",
        "bid_date": null,
        "year": 2021,
        "abc_published": null,
        "prop_takers": null,
        "prop_infra": null,
        "time_extension": null
    }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
    "id": "233",
    "attributes": {
        "name": "Improvement of Hot Spring at Barangay Lourdes\n",
        "code": "DF21-8000-B7-a",
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
        "prop_fund": "2",
        "prop_type": "11",
        "prop_category": "",
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
        "appropriation": 3000000,
        "abc": 0,
        "status_id": {
            "data": null
        },
        "remarks": null,
        "entry_type": "IMPLEMENTED",
        "bid_date": null,
        "year": 2021,
        "abc_published": null,
        "prop_takers": null,
        "prop_infra": null,
        "time_extension": null
    }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
    "id": "234",
    "attributes": {
        "name": "Improvement of Kendis Cave and Const/Dev't of Kindes Cave",
        "code": "DF21-8000-B7-b and DF20-8000-B7g",
        "location": "[\"28\"]",
        "cost": 3916633,
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
        "prop_fund": "2",
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
        "appropriation": 4000000,
        "abc": 3917133.43,
        "status_id": {
            "data": null
        },
        "remarks": "NYS with site problem\n",
        "entry_type": "IMPLEMENTED",
        "bid_date": null,
        "year": 2021,
        "abc_published": null,
        "prop_takers": "YEN-YEN BONTILAO",
        "prop_infra": null,
        "time_extension": null
    }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
    "id": "235",
    "attributes": {
        "name": "Development of Pagadian City Water Park Phase III-Retaining Wall",
        "code": "DF21-8000-B7-c",
        "location": "[\"13\"]",
        "cost": 9690061.36,
        "contractor": {
            "data": {
                "id": "13",
                "attributes": {
                    "name": "Glenson Construction",
                    "street_purok": null
                }
            }
        },
        "duration": 180,
        "adjusted": null,
        "total": null,
        "weight": null,
        "prop_sdg": "[]",
        "prop_fund": "2",
        "prop_type": "11",
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
        "start": "2023-05-30",
        "target": "2023-11-26",
        "office": "Engineer's Office",
        "assigned": null,
        "appropriation": 10000000,
        "abc": 9696468.9,
        "status_id": {
            "data": null
        },
        "remarks": "85% physical accomplishment as per report of project in-charge as of Aug. 2, 2024",
        "entry_type": "IMPLEMENTED",
        "bid_date": "2023-02-06",
        "year": 2021,
        "abc_published": null,
        "prop_takers": null,
        "prop_infra": null,
        "time_extension": null
    }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
    "id": "236",
    "attributes": {
        "name": " Construction of Yatch Docking Area at Pagadian Baywalk\n",
        "code": "DF21-8000-B7-d",
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
        "prop_fund": "2",
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
        "appropriation": 5000000,
        "abc": 0,
        "status_id": {
            "data": null
        },
        "remarks": null,
        "entry_type": "IMPLEMENTED",
        "bid_date": null,
        "year": 2021,
        "abc_published": null,
        "prop_takers": null,
        "prop_infra": null,
        "time_extension": null
    }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
    "id": "237",
    "attributes": {
        "name": "Additional Deevelopment of Pagadian Rotunda\n",
        "code": "DF21-8000-B7-e",
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
        "prop_fund": "2",
        "prop_type": "",
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
        "appropriation": 3000000,
        "abc": 0,
        "status_id": {
            "data": null
        },
        "remarks": null,
        "entry_type": "IMPLEMENTED",
        "bid_date": null,
        "year": 2021,
        "abc_published": null,
        "prop_takers": null,
        "prop_infra": null,
        "time_extension": null
    }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
    "id": "238",
    "attributes": {
        "name": " Improvement of Palpalan View Deck\n",
        "code": "DF21-8000-B7-g",
        "location": "[]",
        "cost": 11580920.58,
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
        "prop_fund": "2",
        "prop_type": "26",
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
        "start": null,
        "target": null,
        "office": "Engineer's Office",
        "assigned": null,
        "appropriation": 5000000,
        "abc": 0,
        "status_id": {
            "data": null
        },
        "remarks": null,
        "entry_type": "IMPLEMENTED",
        "bid_date": "2021-12-27",
        "year": 2021,
        "abc_published": null,
        "prop_takers": null,
        "prop_infra": null,
        "time_extension": null
    }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
    "id": "239",
    "attributes": {
        "name": "Construction of Pagadian International College( PCIC) Building Phase 1",
        "code": "GAD21-3000-A1-d",
        "location": "[\"12\"]",
        "cost": 8231322,
        "contractor": {
            "data": {
                "id": "3",
                "attributes": {
                    "name": "GAF TRADING & CONST",
                    "street_purok": null
                }
            }
        },
        "duration": 120,
        "adjusted": null,
        "total": null,
        "weight": null,
        "prop_sdg": "[]",
        "prop_fund": "4",
        "prop_type": "24",
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
        "start": "2024-03-04",
        "target": "2024-07-02",
        "office": "Engineer's Office",
        "assigned": null,
        "appropriation": 8400000,
        "abc": 8215957.14,
        "status_id": {
            "data": null
        },
        "remarks": "\"* 100% physically completed based in approved plans & specifications\n* with time extension due to Variation Order No. 1\n* Adjusted Target Date of Completion: August 4, 2024\"\n",
        "entry_type": "IMPLEMENTED",
        "bid_date": null,
        "year": 2021,
        "abc_published": null,
        "prop_takers": "DAENA PAMARAN",
        "prop_infra": null,
        "time_extension": null
    }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
    "id": "240",
    "attributes": {
        "name": "Expansion/Improvement of Danlugan Infirmary Complex",
        "code": "GAD21-3000-B6",
        "location": "[\"27\"]",
        "cost": 4748975.24,
        "contractor": {
            "data": {
                "id": "8",
                "attributes": {
                    "name": "JAMT Construction",
                    "street_purok": null
                }
            }
        },
        "duration": 150,
        "adjusted": null,
        "total": null,
        "weight": null,
        "prop_sdg": "[]",
        "prop_fund": "4",
        "prop_type": "26",
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
        "start": "2022-12-15",
        "target": "2023-05-14",
        "office": "Engineer's Office",
        "assigned": null,
        "appropriation": 5000000,
        "abc": 4750000,
        "status_id": {
            "data": null
        },
        "remarks": "100% physically completed based on approved technical plans & specifications\n",
        "entry_type": "IMPLEMENTED",
        "bid_date": "2022-02-23",
        "year": 2021,
        "abc_published": null,
        "prop_takers": null,
        "prop_infra": null,
        "time_extension": null
    }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
    "id": "241",
    "attributes": {
        "name": "Construction of New Senior Citizen Building\n",
        "code": "SCPWD21 - 3000 - A3",
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
        "prop_fund": "10",
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
        "appropriation": 5000000,
        "abc": 0,
        "status_id": {
            "data": null
        },
        "remarks": null,
        "entry_type": "IMPLEMENTED",
        "bid_date": null,
        "year": 2021,
        "abc_published": null,
        "prop_takers": null,
        "prop_infra": null,
        "time_extension": null
    }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
    "id": "242",
    "attributes": {
        "name": "Construction of Proposed PWDBuilding\n",
        "code": "SCPWD21 - 3000 - B4",
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
        "prop_fund": "10",
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
        "appropriation": 5000000,
        "abc": 0,
        "status_id": {
            "data": null
        },
        "remarks": null,
        "entry_type": "IMPLEMENTED",
        "bid_date": null,
        "year": 2021,
        "abc_published": null,
        "prop_takers": null,
        "prop_infra": null,
        "time_extension": null
    }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
    "id": "243",
    "attributes": {
        "name": "Repair/Rehabilitation of Road at Brgy Bulatok",
        "code": "1SAIP21 - 8000R - c - 1a1",
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
        "appropriation": 300000,
        "abc": 0,
        "status_id": {
            "data": null
        },
        "remarks": null,
        "entry_type": "IMPLEMENTED",
        "bid_date": null,
        "year": 2021,
        "abc_published": null,
        "prop_takers": null,
        "prop_infra": null,
        "time_extension": null
    }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
    "id": "244",
    "attributes": {
        "name": "Road Opening at Brgy Kawit\n",
        "code": "1SAIP21-8000R-c-1a2",
        "location": "[]",
        "cost": 480375,
        "contractor": {
            "data": {
                "id": "5",
                "attributes": {
                    "name": "MOLROW CONSTRUCTION",
                    "street_purok": null
                }
            }
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
        "bid_date": "2021-12-08",
        "year": 2021,
        "abc_published": null,
        "prop_takers": null,
        "prop_infra": null,
        "time_extension": null
    }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
    "id": "245",
    "attributes": {
        "name": "Road Opening/Clearing at Barangay Sta. Lucia",
        "code": "1SAIP21-8000RB-c-1a3",
        "location": "[]",
        "cost": 386735,
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
        "prop_fund": "5",
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
        "bid_date": "2022-03-17",
        "year": 2021,
        "abc_published": null,
        "prop_takers": null,
        "prop_infra": null,
        "time_extension": null
    }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
    "id": "246",
    "attributes": {
        "name": "Road Concreting at Barangay Balintawak (RMG)",
        "code": "1SAIP21-8000RB-c-1a4",
        "location": "[]",
        "cost": 1085352.84,
        "contractor": {
            "data": {
                "id": "29",
                "attributes": {
                    "name": "Genetian Builders",
                    "street_purok": null
                }
            }
        },
        "duration": null,
        "adjusted": null,
        "total": null,
        "weight": null,
        "prop_sdg": "[]",
        "prop_fund": "5",
        "prop_type": "1",
        "prop_category": "2",
        "prop_sector": "[]",
        "prop_assign": "2",
        "accom_total": 1000,
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
        "appropriation": 1100000,
        "abc": 0,
        "status_id": {
            "data": null
        },
        "remarks": null,
        "entry_type": "IMPLEMENTED",
        "bid_date": "2021-01-30",
        "year": 2021,
        "abc_published": null,
        "prop_takers": null,
        "prop_infra": null,
        "time_extension": null
    }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
    "id": "247",
    "attributes": {
        "name": "Concreting/Improvement of Road at Lacturan, Barangay San Pedro\n",
        "code": "1SAIP21-8000RB-c-1a5",
        "location": "[]",
        "cost": 484354,
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
        "prop_fund": "5",
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
        "bid_date": "2021-06-23",
        "year": 2021,
        "abc_published": null,
        "prop_takers": null,
        "prop_infra": null,
        "time_extension": null
    }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
    "id": "248",
    "attributes": {
        "name": "mprovement of Road Leading to Nazareth Road at JCT Natrional Highway Crossing (Additional )",
        "code": "1SAIP21-8000RB-C-1a6 (JOINT RF22-8000-3A-5)",
        "location": "[\"4\"]",
        "cost": 767513.78,
        "contractor": {
            "data": {
                "id": "4",
                "attributes": {
                    "name": "Alajeño Construction",
                    "street_purok": null
                }
            }
        },
        "duration": 90,
        "adjusted": null,
        "total": null,
        "weight": null,
        "prop_sdg": "[]",
        "prop_fund": "5",
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
        "start": "2023-03-20",
        "target": "2023-06-18",
        "office": "Engineer's Office",
        "assigned": null,
        "appropriation": 300000,
        "abc": 0,
        "status_id": {
            "data": null
        },
        "remarks": "\"* 100% physically completed based on approved technical plans & specifications\n* w/ time extension as per approved SO No.1\n* Adjusted Target Date of Completion: Dec. 15, 2023\"\n",
        "entry_type": "IMPLEMENTED",
        "bid_date": "2023-01-23",
        "year": 2021,
        "abc_published": null,
        "prop_takers": null,
        "prop_infra": null,
        "time_extension": null
    }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
    "id": "249",
    "attributes": {
        "name": "Unfinished Portion (Gaps) Between Constructued Sidewalks and Existing Road Pavement at Barangays San Francisco, San Jose, Gatas, Balangasan, Sta Lucia, Santiago, Sta. Maria\n",
        "code": "1SAIP21-8000RB-c-1a7",
        "location": "[]",
        "cost": 4849342.7,
        "contractor": {
            "data": {
                "id": "19",
                "attributes": {
                    "name": "Jenrich Construction",
                    "street_purok": null
                }
            }
        },
        "duration": 150,
        "adjusted": null,
        "total": null,
        "weight": null,
        "prop_sdg": "[]",
        "prop_fund": "5",
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
        "start": "2021-10-07",
        "target": "2022-03-06",
        "office": "Engineer's Office",
        "assigned": null,
        "appropriation": 5000000,
        "abc": 0,
        "status_id": {
            "data": null
        },
        "remarks": "100% physically completed\n",
        "entry_type": "IMPLEMENTED",
        "bid_date": "2021-08-31",
        "year": 2021,
        "abc_published": null,
        "prop_takers": null,
        "prop_infra": null,
        "time_extension": null
    }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
            "id": "250",
            "attributes": {
                "name": "Repair/Rehabilitation of River Control at Gatas Bridge, Urro St\n",
                "code": "1SAIP21-8000R-C-1b1",
                "location": "[]",
                "cost": 288313.64,
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
                "prop_fund": "5",
                "prop_type": "8",
                "prop_category": "3",
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
                "remarks": null,
                "entry_type": "IMPLEMENTED",
                "bid_date": "2021-07-06",
                "year": 2021,
                "abc_published": null,
                "prop_takers": null,
                "prop_infra": null,
                "time_extension": null
            }
            }), db_connection).await,
convert_projects_into_active_model_v2(&json!({
            "id": "251",
            "attributes": {
                "name": "Completion of the Construction of Underground Drainage and Sidewalk Along Jamisola St St.\n",
                "code": "1SAIP21-8000R-C-1b2",
                "location": "[]",
                "cost": 125039.38,
                "contractor": {
                "data": {
                    "id": "32",
                    "attributes": {
                    "name": "Pagadian Aggregates",
                    "street_purok": null
                    }
                }
                },
                "duration": null,
                "adjusted": null,
                "total": null,
                "weight": null,
                "prop_sdg": "[]",
                "prop_fund": "5",
                "prop_type": "7",
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
                "appropriation": 130000,
                "abc": 0,
                "status_id": {
                "data": null
                },
                "remarks": null,
                "entry_type": "IMPLEMENTED",
                "bid_date": "2021-08-20",
                "year": 2021,
                "abc_published": null,
                "prop_takers": null,
                "prop_infra": null,
                "time_extension": null
            }
            }), db_connection).await,
convert_projects_into_active_model_v2(&json!({
            "id": "252",
            "attributes": {
                "name": " Construction of Drainage Across National Highway & FS Pajares Ave",
                "code": "1SAIP21 - 8000R - C - 1b3",
                "location": null,
                "cost": 1250254.72,
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
                "prop_sdg": null,
                "prop_fund": "5",
                "prop_type": null,
                "prop_category": null,
                "prop_sector": null,
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
                "appropriation": 1300000,
                "abc": 0,
                "status_id": {
                "data": null
                },
                "remarks": null,
                "entry_type": "IMPLEMENTED",
                "bid_date": "2021-12-06",
                "year": 2021,
                "abc_published": null,
                "prop_takers": null,
                "prop_infra": null,
                "time_extension": null
            }
            }), db_connection).await,
convert_projects_into_active_model_v2(&json!({
            "id": "253",
            "attributes": {
                "name": "Construction of Hanging Bridge at Barangay Balangasan\n",
                "code": "1SAIP21 - 8000R - C - 1b4",
                "location": null,
                "cost": 145108.5,
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
                "prop_sdg": null,
                "prop_fund": "5",
                "prop_type": null,
                "prop_category": null,
                "prop_sector": null,
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
                "bid_date": "2021-06-23",
                "year": 2021,
                "abc_published": null,
                "prop_takers": null,
                "prop_infra": null,
                "time_extension": null
            }
            }), db_connection).await,
convert_projects_into_active_model_v2(&json!({
            "id": "254",
            "attributes": {
                "name": " Completion of the Construction of Police Outpost (Phase 2)\nTiguma Boundary\n",
                "code": "1SAIP21 - 3000R - C - 1c1 - a",
                "location": null,
                "cost": 0,
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
                "prop_sdg": null,
                "prop_fund": "5",
                "prop_type": null,
                "prop_category": null,
                "prop_sector": null,
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
                "appropriation": 350000,
                "abc": 0,
                "status_id": {
                "data": null
                },
                "remarks": " SITE PROBLEM \n",
                "entry_type": "IMPLEMENTED",
                "bid_date": "2021-07-20",
                "year": 2021,
                "abc_published": null,
                "prop_takers": null,
                "prop_infra": null,
                "time_extension": null
            }
            }), db_connection).await,
convert_projects_into_active_model_v2(&json!({
            "id": "255",
            "attributes": {
                "name": "Completion of the Construction of Police Outpost (Phase 2) Buenavista - Baloyboan\n\n",
                "code": "1SAIP21 - 3000R - C - 1c1 - b",
                "location": null,
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
                "duration": null,
                "adjusted": null,
                "total": null,
                "weight": null,
                "prop_sdg": null,
                "prop_fund": "5",
                "prop_type": null,
                "prop_category": null,
                "prop_sector": null,
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
                "appropriation": 350000,
                "abc": 0,
                "status_id": {
                "data": null
                },
                "remarks": null,
                "entry_type": "IMPLEMENTED",
                "bid_date": "2021-09-21",
                "year": 2021,
                "abc_published": null,
                "prop_takers": null,
                "prop_infra": null,
                "time_extension": null
            }
            }), db_connection).await,
convert_projects_into_active_model_v2(&json!({
            "id": "256",
            "attributes": {
                "name": "Completion of the Construction of Police Outpost (Phase 2) Macasing Crossing\n\n",
                "code": "1SAIP21 - 3000R - C - 1c1 - c",
                "location": null,
                "cost": 0,
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
                "prop_sdg": null,
                "prop_fund": "5",
                "prop_type": null,
                "prop_category": null,
                "prop_sector": null,
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
                "remarks": "SITE PROBLEM",
                "entry_type": "IMPLEMENTED",
                "bid_date": "2021-07-20",
                "year": 2021,
                "abc_published": null,
                "prop_takers": null,
                "prop_infra": null,
                "time_extension": null
            }
            }), db_connection).await,
convert_projects_into_active_model_v2(&json!({
            "id": "257",
            "attributes": {
                "name": "Completion of the Construction of Police Outpost (Phase 2) d. Danlugan\n\n",
                "code": "1SAIP21 - 3000R - C - 1c1 - d",
                "location": null,
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
                "duration": null,
                "adjusted": null,
                "total": null,
                "weight": null,
                "prop_sdg": null,
                "prop_fund": "5",
                "prop_type": null,
                "prop_category": null,
                "prop_sector": null,
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
                "bid_date": "2021-07-20",
                "year": 2021,
                "abc_published": null,
                "prop_takers": null,
                "prop_infra": null,
                "time_extension": null
            }
            }), db_connection).await,
convert_projects_into_active_model_v2(&json!({
            "id": "258",
            "attributes": {
                "name": "Completion of the Construction of Police Outpost (Phase 2),  e. Poloyagan",
                "code": "1SAIP21-3000R-C-1c1-e ( 150K) /RF20-1000-2A-1j-7 (joint fund)(700K)",
                "location": "[\"49\"]",
                "cost": 818044.19,
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
                "prop_fund": "5",
                "prop_type": "14",
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
                "start": "2023-04-24",
                "target": "2023-07-23",
                "office": "Engineer's Office",
                "assigned": null,
                "appropriation": 150000,
                "abc": 0,
                "status_id": {
                "data": null
                },
                "remarks": "\"* 100% physically completed based on approved technical plans & specifications\n*with time extension as per Suspension Order \n* Adjusted Target Completion Date: Jan. 29, 2024\n* Actual Completion Date:  Jan.26, 2024\"\n",
                "entry_type": "IMPLEMENTED",
                "bid_date": null,
                "year": 2021,
                "abc_published": null,
                "prop_takers": "CAPT. DACAL",
                "prop_infra": null,
                "time_extension": null
            }
            }), db_connection).await,
convert_projects_into_active_model_v2(&json!({
            "id": "259",
            "attributes": {
                "name": " Repair/Rehabilitation of City PNP Building\n",
                "code": "1SAIP21 - 3000R - C - 1c2",
                "location": null,
                "cost": 717335.4,
                "contractor": {
                "data": {
                    "id": "33",
                    "attributes": {
                    "name": "GCMG Construction",
                    "street_purok": null
                    }
                }
                },
                "duration": null,
                "adjusted": null,
                "total": null,
                "weight": null,
                "prop_sdg": null,
                "prop_fund": "5",
                "prop_type": null,
                "prop_category": null,
                "prop_sector": null,
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
                "appropriation": 350000,
                "abc": 0,
                "status_id": {
                "data": null
                },
                "remarks": null,
                "entry_type": "IMPLEMENTED",
                "bid_date": "2021-09-07",
                "year": 2021,
                "abc_published": null,
                "prop_takers": null,
                "prop_infra": null,
                "time_extension": null
            }
            }), db_connection).await,
            convert_projects_into_active_model_v2(&json!({
            "id": "260",
            "attributes": {
                "name": " Construction of School Stage at Tiguma E/S\n",
                "code": "1SAIP21 - 3000R - C - 1c3",
                "location": null,
                "cost": 1272949.1,
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
                "prop_sdg": null,
                "prop_fund": "5",
                "prop_type": null,
                "prop_category": null,
                "prop_sector": null,
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
                "appropriation": 1330000,
                "abc": 0,
                "status_id": {
                "data": null
                },
                "remarks": null,
                "entry_type": "IMPLEMENTED",
                "bid_date": "2021-09-21",
                "year": 2021,
                "abc_published": null,
                "prop_takers": null,
                "prop_infra": null,
                "time_extension": null
            }
            }), db_connection).await,
convert_projects_into_active_model_v2(&json!({
            "id": "261",
            "attributes": {
                "name": "Construction of PHILPOST Building\n",
                "code": "1SAIP21 - 3000R - C - 1c5",
                "location": "[]",
                "cost": 9529580.7,
                "contractor": {
                "data": {
                    "id": "34",
                    "attributes": {
                    "name": "Abdulbasit Construction",
                    "street_purok": null
                    }
                }
                },
                "duration": null,
                "adjusted": null,
                "total": null,
                "weight": null,
                "prop_sdg": "[]",
                "prop_fund": "5",
                "prop_type": "26",
                "prop_category": "1",
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
                "appropriation": 10000000,
                "abc": 0,
                "status_id": {
                "data": null
                },
                "remarks": null,
                "entry_type": "IMPLEMENTED",
                "bid_date": "2021-11-08",
                "year": 2021,
                "abc_published": null,
                "prop_takers": null,
                "prop_infra": null,
                "time_extension": null
            }
            }), db_connection).await,
convert_projects_into_active_model_v2(&json!({
            "id": "262",
            "attributes": {
                "name": " Construction of PENRO Building at Barangay Dao\n",
                "code": "1SAIP21 - 3000R - C - 1c6",
                "location": null,
                "cost": 0,
                "contractor": {
                "data": null
                },
                "duration": null,
                "adjusted": null,
                "total": null,
                "weight": null,
                "prop_sdg": null,
                "prop_fund": "5",
                "prop_type": null,
                "prop_category": null,
                "prop_sector": null,
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
                "appropriation": 12000000,
                "abc": 0,
                "status_id": {
                "data": null
                },
                "remarks": null,
                "entry_type": "IMPLEMENTED",
                "bid_date": null,
                "year": 2021,
                "abc_published": null,
                "prop_takers": null,
                "prop_infra": null,
                "time_extension": null
            }
            }), db_connection).await,
convert_projects_into_active_model_v2(&json!({
            "id": "263",
            "attributes": {
                "name": "Construction of SP Building Phase II\n",
                "code": "1SAIP21 - 8000R - C - 1c7",
                "location": "[\"12\"]",
                "cost": 14375046.31,
                "contractor": {
                "data": {
                    "id": "35",
                    "attributes": {
                    "name": "JH Pameron Construction",
                    "street_purok": null
                    }
                }
                },
                "duration": 190,
                "adjusted": null,
                "total": null,
                "weight": null,
                "prop_sdg": "[]",
                "prop_fund": "5",
                "prop_type": "26",
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
                "start": "2023-05-12",
                "target": "2023-11-18",
                "office": "Engineer's Office",
                "assigned": null,
                "appropriation": 15000000,
                "abc": 0,
                "status_id": {
                "data": null
                },
                "remarks": "85% PHYSICAL ACCOMPLISHMENT AS OF DEC. 10, 2023",
                "entry_type": "IMPLEMENTED",
                "bid_date": null,
                "year": 2021,
                "abc_published": null,
                "prop_takers": "ENGR. PAMERON",
                "prop_infra": null,
                "time_extension": null
            }
            }), db_connection).await,
convert_projects_into_active_model_v2(&json!({
            "id": "264",
            "attributes": {
                "name": "Construction of CSWDO Building (Phase I)\n",
                "code": "1SAIP21 - 8000R - C - 1c8",
                "location": "[\"12\"]",
                "cost": 14280656.95,
                "contractor": {
                "data": {
                    "id": "8",
                    "attributes": {
                    "name": "JAMT Construction",
                    "street_purok": null
                    }
                }
                },
                "duration": 90,
                "adjusted": null,
                "total": null,
                "weight": null,
                "prop_sdg": "[]",
                "prop_fund": "5",
                "prop_type": "26",
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
                "start": "2022-03-08",
                "target": "2022-06-06",
                "office": "Engineer's Office",
                "assigned": null,
                "appropriation": 15000000,
                "abc": 0,
                "status_id": {
                "data": null
                },
                "remarks": "80% physical accomplishment as of DEC. 30, 2023",
                "entry_type": "IMPLEMENTED",
                "bid_date": "2021-12-17",
                "year": 2021,
                "abc_published": null,
                "prop_takers": "JOMARC & FERDIE",
                "prop_infra": null,
                "time_extension": null
            }
            }), db_connection).await,
convert_projects_into_active_model_v2(&json!({
            "id": "265",
            "attributes": {
                "name": "Improvement of C3 Building (Portion Only) for DFA Office\n",
                "code": "1SAIP21 - 8000R - C - 1c9",
                "location": null,
                "cost": 0,
                "contractor": {
                "data": null
                },
                "duration": null,
                "adjusted": null,
                "total": null,
                "weight": null,
                "prop_sdg": null,
                "prop_fund": "5",
                "prop_type": null,
                "prop_category": null,
                "prop_sector": null,
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
                "year": 2021,
                "abc_published": null,
                "prop_takers": null,
                "prop_infra": null,
                "time_extension": null
            }
            }), db_connection).await,
convert_projects_into_active_model_v2(&json!({
            "id": "266",
            "attributes": {
                "name": "Improvement of C3 Building (Portion Only) for PSA Office\n",
                "code": "1SAIP21 - 8000R - C - 1c10",
                "location": null,
                "cost": 0,
                "contractor": {
                "data": null
                },
                "duration": null,
                "adjusted": null,
                "total": null,
                "weight": null,
                "prop_sdg": null,
                "prop_fund": "5",
                "prop_type": null,
                "prop_category": null,
                "prop_sector": null,
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
                "year": 2021,
                "abc_published": null,
                "prop_takers": null,
                "prop_infra": null,
                "time_extension": null
            }
            }), db_connection).await,
convert_projects_into_active_model_v2(&json!({
            "id": "267",
            "attributes": {
                "name": "Improvement/Additional Works at Covered Court at City Plaza",
                "code": "1SAIP21 - 8000R - C - 1c11",
                "location": "[]",
                "cost": 2752627.71,
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
                "prop_fund": "5",
                "prop_type": "16",
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
                "appropriation": 2850000,
                "abc": 0,
                "status_id": {
                "data": null
                },
                "remarks": "90% physical accomplishment as of oct. 21, 2023 report",
                "entry_type": "IMPLEMENTED",
                "bid_date": "2022-03-08",
                "year": 2021,
                "abc_published": null,
                "prop_takers": null,
                "prop_infra": null,
                "time_extension": null
            }
            }), db_connection).await,
convert_projects_into_active_model_v2(&json!({
            "id": "268",
            "attributes": {
                "name": "Add'l Painting Works at the Constructed Evacuation Center at Brgy Dao\n",
                "code": "1SAIP21 - 8000R - C - 1c12",
                "location": null,
                "cost": 772325.57,
                "contractor": {
                "data": {
                    "id": "12",
                    "attributes": {
                    "name": "LC Construction",
                    "street_purok": null
                    }
                }
                },
                "duration": null,
                "adjusted": null,
                "total": null,
                "weight": null,
                "prop_sdg": null,
                "prop_fund": "5",
                "prop_type": null,
                "prop_category": null,
                "prop_sector": null,
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
                "appropriation": 800000,
                "abc": 0,
                "status_id": {
                "data": null
                },
                "remarks": null,
                "entry_type": "IMPLEMENTED",
                "bid_date": "2021-09-08",
                "year": 2021,
                "abc_published": null,
                "prop_takers": null,
                "prop_infra": null,
                "time_extension": null
            }
            }), db_connection).await,
convert_projects_into_active_model_v2(&json!({
            "id": "269",
            "attributes": {
                "name": "Completion of Const of Perimeter Fence at Livestock Bagsakan, Brgy San Pedro\n",
                "code": "1SAIP21 - 8000R - C - 1c13",
                "location": null,
                "cost": 397875.26,
                "contractor": {
                "data": {
                    "id": "12",
                    "attributes": {
                    "name": "LC Construction",
                    "street_purok": null
                    }
                }
                },
                "duration": null,
                "adjusted": null,
                "total": null,
                "weight": null,
                "prop_sdg": null,
                "prop_fund": "5",
                "prop_type": null,
                "prop_category": null,
                "prop_sector": null,
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
                "bid_date": "2021-07-06",
                "year": 2021,
                "abc_published": null,
                "prop_takers": null,
                "prop_infra": null,
                "time_extension": null
            }
            }), db_connection).await,
            convert_projects_into_active_model_v2(&json!({
            "id": "270",
            "attributes": {
                "name": "Development of Farm Center and Piggery Farm\n",
                "code": "1SAIP21 - 8000R - C - 1c14",
                "location": null,
                "cost": 0,
                "contractor": {
                "data": null
                },
                "duration": null,
                "adjusted": null,
                "total": null,
                "weight": null,
                "prop_sdg": null,
                "prop_fund": "5",
                "prop_type": null,
                "prop_category": null,
                "prop_sector": null,
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
                "year": 2021,
                "abc_published": null,
                "prop_takers": null,
                "prop_infra": null,
                "time_extension": null
            }
            }), db_connection).await,
convert_projects_into_active_model_v2(&json!({
            "id": "271",
            "attributes": {
                "name": "Purchase of Lot for Regional Center at Barangay Balintawak\n",
                "code": "1SAIP21 - 8000R - C - 1d1",
                "location": null,
                "cost": 0,
                "contractor": {
                "data": null
                },
                "duration": null,
                "adjusted": null,
                "total": null,
                "weight": null,
                "prop_sdg": null,
                "prop_fund": "5",
                "prop_type": null,
                "prop_category": null,
                "prop_sector": null,
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
                "appropriation": 300000,
                "abc": 0,
                "status_id": {
                "data": null
                },
                "remarks": null,
                "entry_type": "IMPLEMENTED",
                "bid_date": null,
                "year": 2021,
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