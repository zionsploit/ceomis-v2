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

        let make_add = [
            convert_projects_into_active_model_v2(&json!({
            "id": "11",
            "attributes": {
                "name": "Installation/Establishment of Street Names (All Street Corners)",
                "code": "RF22-1000-1-2B-1c",
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
                "prop_category": "",
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
                "start": "Invalid date",
                "target": "Invalid date",
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
                "year": 2022,
                "abc_published": null,
                "prop_takers": null,
                "prop_infra": null,
                "time_extension": null,
                "project_status": "Preparation"
            }
            }), db_connection).await,
            convert_projects_into_active_model_v2(&json!({
            "id": "13",
            "attributes": {
                "project_status": "Completed",
                "name": "Completion for the Const. of Purok Subida Road Alley , Barangay Dao",
                "code": "RF22-8000-3A-3",
                "location": "[\"5\"]",
                "cost": 480734,
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
                "prop_fund": "1",
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
                "start": "Invalid date",
                "target": "Invalid date",
                "office": "Engineer's Office",
                "assigned": null,
                "appropriation": 500000,
                "abc": 481526.63,
                "status_id": {
                "data": null
                },
                "remarks": "",
                "entry_type": "IMPLEMENTED",
                "bid_date": "2023-01-04",
                "year": 2022,
                "abc_published": null,
                "prop_takers": null,
                "prop_infra": null,
                "time_extension": null
            }
            }), db_connection).await,
            convert_projects_into_active_model_v2(&json!({
                "id": "14",
                "attributes": {
                    "project_status": "Completed",
                    "name": "Improvement/Widening of Road at Buenavista from Jct. National Highway Leading to Nazareth",
                    "code": "RF22-8000-3A-5 (JOINT 1SAIP21-8000RB-C-1a6 )",
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
                    "start": "2023-03-30",
                    "target": "2023-06-28",
                    "office": "Engineer's Office",
                    "assigned": null,
                    "appropriation": 500000,
                    "abc": 768458.88,
                    "status_id": {
                    "data": null
                    },
                    "remarks": "\"* 100% physically completed based on approved technical plans & specifications\n* w/ time extension as per approved SO No.1\n* Adjusted Target Date of Completion: Dec. 15, 2023\"\n",
                    "entry_type": "IMPLEMENTED",
                    "bid_date": "2023-01-23",
                    "year": 2022,
                    "abc_published": null,
                    "prop_takers": "RIC ALAJENO",
                    "prop_infra": null,
                    "time_extension": null
                }
                }), db_connection).await,
            convert_projects_into_active_model_v2(&json!({
          "id": "15",
          "attributes": {
            "project_status": "On-Going",
            "name": "Rehabilitation/Construction of Slaughterhouse at Barangay Napolan (Including Purchase of Equipment)",
            "code": "RF22-8000-3B-1",
            "location": "[\"10\"]",
            "cost": 9692233.74,
            "contractor": {
              "data": {
                "id": "4",
                "attributes": {
                  "name": "Alajeño Construction",
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
            "prop_type": "4",
            "prop_category": "3",
            "prop_sector": "[]",
            "prop_assign": "2",
            "accom_total": 43,
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
            "target": "Invalid date",
            "office": "Engineer's Office",
            "assigned": null,
            "appropriation": 10000000,
            "abc": 9696515.67,
            "status_id": {
              "data": null
            },
            "remarks": "On-going implementation of he project w/ 43% physical accomplishment as of  March 8, 2024\n",
            "entry_type": "IMPLEMENTED",
            "bid_date": "2023-04-17",
            "year": 2022,
            "abc_published": null,
            "prop_takers": "RIC ALAJENO",
            "prop_infra": null,
            "time_extension": null
          }
        }), db_connection).await,
        convert_projects_into_active_model_v2(&json!({
          "id": "16",
          "attributes": {
            "project_status": "Completed",
            "name": "Concreting of Road Leading to Manga Falls (Unfinished portion)\n",
            "code": "DF22-8000-B1a",
            "location": "[\"45\"]",
            "cost": 2890753,
            "contractor": {
              "data": {
                "id": "1",
                "attributes": {
                  "name": "CTG Construction",
                  "street_purok": "Purok 1"
                }
              }
            },
            "duration": 98,
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
            "start": "2022-11-07",
            "target": "2023-03-23",
            "office": "Engineer's Office",
            "assigned": null,
            "appropriation": 3000000,
            "abc": 2891334.27,
            "status_id": {
              "data": null
            },
            "remarks": "",
            "entry_type": "IMPLEMENTED",
            "bid_date": "2022-09-14",
            "year": 2022,
            "abc_published": null,
            "prop_takers": null,
            "prop_infra": null,
            "time_extension": null
          }
        }), db_connection).await,
        convert_projects_into_active_model_v2(&json!({
          "id": "17",
          "attributes": {
            "project_status": "Completed",
            "name": "Concreting of Road Leading to Kawit E/S\n",
            "code": "DF22-8000-B1b",
            "location": null,
            "cost": 479778.84,
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
            "prop_sdg": null,
            "prop_fund": "2",
            "prop_type": null,
            "prop_category": null,
            "prop_sector": null,
            "prop_assign": null,
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
            "status": "For Preparation",
            "start": null,
            "target": null,
            "office": "Engineer's Office",
            "assigned": "Eng. Perzeus Developer",
            "appropriation": 500000,
            "abc": 0,
            "status_id": {
              "data": null
            },
            "remarks": null,
            "entry_type": "IMPLEMENTED",
            "bid_date": "2022-07-18",
            "year": 2022,
            "abc_published": null,
            "prop_takers": null,
            "prop_infra": null,
            "time_extension": null
          }
        }), db_connection).await,
        convert_projects_into_active_model_v2(&json!({
          "id": "18",
          "attributes": {
            "project_status": "Completed",
            "name": "Concreting of Pathway at White Beach \n",
            "code": "DF22-8000-B1d",
            "location": "[\"54\"]",
            "cost": 961466.75,
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
            "start": "2023-04-01",
            "target": "2023-05-16",
            "office": "Engineer's Office",
            "assigned": null,
            "appropriation": 1000000,
            "abc": 0,
            "status_id": {
              "data": null
            },
            "remarks": "* 100% physically completed based on approved technical plans & specifications\n* w/ time extension due to approved VO No.1\n* Adjusted Target time of completion: Oct. 1, 2023\n* Actual Date Completed: 9/29/2023\n",
            "entry_type": "IMPLEMENTED",
            "bid_date": "2023-01-04",
            "year": 2022,
            "abc_published": null,
            "prop_takers": "TENG MARCABAN",
            "prop_infra": null,
            "time_extension": null
          }
        }), db_connection).await,
        convert_projects_into_active_model_v2(&json!({
          "id": "19",
          "attributes": {
            "project_status": "Completed",
            "name": "Concreting of Road (Gaps between existing road and sidewalk)\n",
            "code": "DF22-8000-B1e",
            "location": "[]",
            "cost": 9697293.28,
            "contractor": {
              "data": {
                "id": "7",
                "attributes": {
                  "name": "St. Joseph Construction",
                  "street_purok": null
                }
              }
            },
            "duration": 120,
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
            "start": "2023-01-19",
            "target": "2023-05-19",
            "office": "Engineer's Office",
            "assigned": null,
            "appropriation": 10000000,
            "abc": 9698985.11,
            "status_id": {
              "data": null
            },
            "remarks": "\"* 100% Physically completed based on approved plans & specifications\n* with time extension as per approved suspension order\"\n",
            "entry_type": "IMPLEMENTED",
            "bid_date": "2022-11-14",
            "year": 2022,
            "abc_published": null,
            "prop_takers": "\"CHAMPION IPIL- ELENA-09278305516\"",
            "prop_infra": null,
            "time_extension": null
          }
        }), db_connection).await,
        convert_projects_into_active_model_v2(&json!({
          "id": "20",
          "attributes": {
            "project_status": "Completed",
            "name": "Pagadian Infirmary Perimeter Fence",
            "code": "DF22-8000-B5-d",
            "location": "[\"27\"]",
            "cost": 481498.1,
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
            "prop_fund": "2",
            "prop_type": "27",
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
            "start": "Invalid date",
            "target": "Invalid date",
            "office": "Engineer's Office",
            "assigned": null,
            "appropriation": 500000,
            "abc": 0,
            "status_id": {
              "data": null
            },
            "remarks": "",
            "entry_type": "IMPLEMENTED",
            "bid_date": "2022-09-28",
            "year": 2022,
            "abc_published": null,
            "prop_takers": null,
            "prop_infra": null,
            "time_extension": null
          }
        }), db_connection).await,
        convert_projects_into_active_model_v2(&json!({
          "id": "21",
          "attributes": {
            "project_status": "Not Yet Started",
            "name": "Widening of Urro St. (Jct. R.T Lim to Consolation St.) Unfinished portion\n",
            "code": "DF22-8000-B1g",
            "location": "[]",
            "cost": 290327.22,
            "contractor": {
              "data": {
                "id": "2",
                "attributes": {
                  "name": "ALZ Construction",
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
            "bid_date": "2022-09-08",
            "year": 2022,
            "abc_published": null,
            "prop_takers": null,
            "prop_infra": null,
            "time_extension": null
          }
        }), db_connection).await,
        convert_projects_into_active_model_v2(&json!({
          "id": "22",
          "attributes": {
            "project_status": "Completed",
            "name": "Concreting/Improvement of Road and Drainage System fronting main Gate at Palasyo, Sta. Lucia",
            "code": "DF22-8000-B1h",
            "location": "[\"14\"]",
            "cost": 2863172.33,
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
            "start": "2023-03-17",
            "target": "2023-08-14",
            "office": "Engineer's Office",
            "assigned": null,
            "appropriation": 3000000,
            "abc": 0,
            "status_id": {
              "data": null
            },
            "remarks": "\"* 100% physically completed base on approved technical plans & specifications\n* w/ time extension as per approved Suspension Order No. 1\n* Adjusted Targe Date of Completion: Nov. 17, 2023\"\n* Actual Date Completed: 11/17/2023\n",
            "entry_type": "IMPLEMENTED",
            "bid_date": "2023-01-17",
            "year": 2022,
            "abc_published": null,
            "prop_takers": "YENYEN BONTILAO",
            "prop_infra": null,
            "time_extension": null
          }
        }), db_connection).await,
        convert_projects_into_active_model_v2(&json!({
          "id": "23",
          "attributes": {
            "project_status": "On-Going",
            "name": "Continuation of the Construction of Water Park-Phase III, San Pedro",
            "code": "DF22-8000-B8-b",
            "location": "[\"13\"]",
            "cost": 7687337.3,
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
            "prop_fund": "2",
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
            "start": "2023-01-12",
            "target": "2023-05-12",
            "office": "Engineer's Office",
            "assigned": null,
            "appropriation": 8000000,
            "abc": 7745068.89,
            "status_id": {
              "data": null
            },
            "remarks": "On-going implementation of the project\n",
            "entry_type": "IMPLEMENTED",
            "bid_date": "2022-09-19",
            "year": 2022,
            "abc_published": null,
            "prop_takers": "Jun Mondarte",
            "prop_infra": null,
            "time_extension": null
          }
        }), db_connection).await,
        convert_projects_into_active_model_v2(&json!({
          "id": "24",
          "attributes": {
            "project_status": "On-Going",
            "name": "Construction of Sidewalk and Underground Drainage Along Urro St. (Jct. F.S Pajares Ave.-Alano St.) Northern Side",
            "code": "DRRMF22-1000-2-a1",
            "location": "[\"12\"]",
            "cost": 2908331.74,
            "contractor": {
              "data": {
                "id": "6",
                "attributes": {
                  "name": "DM Ventures",
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
            "prop_type": "7",
            "prop_category": "1",
            "prop_sector": "[]",
            "prop_assign": "2",
            "accom_total": 76,
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
            "start": "2023-03-16",
            "target": "2023-07-24",
            "office": "Engineer's Office",
            "assigned": null,
            "appropriation": 3000000,
            "abc": 2909180.78,
            "status_id": {
              "data": null
            },
            "remarks": "Temporarily stopped due to road right of way problem (Jack&Jill)\n\nHas resumed the work & with 71% physical accomplishment as of Nov. 29, 2024\n\n*76% accomplishment as of dec. 6, 2024",
            "entry_type": "IMPLEMENTED",
            "bid_date": "2023-01-17",
            "year": 2022,
            "abc_published": null,
            "prop_takers": null,
            "prop_infra": null,
            "time_extension": null
          }
        }), db_connection).await,
        convert_projects_into_active_model_v2(&json!({
          "id": "25",
          "attributes": {
            "project_status": "On-Going",
            "name": "Construction of Sidewalk and Underground Drainage Along Aquino St. (Jct.Sabate St. -Salazar St.) Western Side",
            "code": "DRRMF22-1000-2-a2",
            "location": "[\"12\"]",
            "cost": 4836730.52,
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
            "accom_total": 55,
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
            "remarks": "55% PHYSICAL ACCOMPLISHMENT AS OF NOV. 10, 2023",
            "entry_type": "IMPLEMENTED",
            "bid_date": "2022-08-15",
            "year": 2022,
            "abc_published": null,
            "prop_takers": null,
            "prop_infra": null,
            "time_extension": null
          }
        }), db_connection).await,
        convert_projects_into_active_model_v2(&json!({
          "id": "26",
          "attributes": {
            "project_status": "Completed",
            "name": "Construction of Sidewalk and Underground Drainage Along F.S Pajares Ave. (Jct. Sagun St. - Sanson St.) Western Side",
            "code": "DRRMF22-1000-2-a4",
            "location": "[]",
            "cost": 1445698.08,
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
            "appropriation": 1500000,
            "abc": 0,
            "status_id": {
              "data": null
            },
            "remarks": "",
            "entry_type": "IMPLEMENTED",
            "bid_date": "2022-12-20",
            "year": 2022,
            "abc_published": null,
            "prop_takers": null,
            "prop_infra": null,
            "time_extension": null
          }
        }), db_connection).await,
        convert_projects_into_active_model_v2(&json!({
          "id": "27",
          "attributes": {
            "project_status": "Not Yet Started",
            "name": "Construction of Sidewalk and Underground Drainage Along F.S Pajares Ave. (Jct.Cabrera St.- National Highway)Western Side",
            "code": "DRRMF22-1000-2-a5",
            "location": "[]",
            "cost": 1459715.05,
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
            "appropriation": 1500000,
            "abc": 1460914.68,
            "status_id": {
              "data": null
            },
            "remarks": "Bidded but still not  started\n",
            "entry_type": "IMPLEMENTED",
            "bid_date": "2022-12-27",
            "year": 2022,
            "abc_published": null,
            "prop_takers": null,
            "prop_infra": null,
            "time_extension": null
          }
        }), db_connection).await,
        convert_projects_into_active_model_v2(&json!({
          "id": "28",
          "attributes": {
            "project_status": "On-Going",
            "name": "Construction of Sidewalk and Underground drainage along Ariosa St.( Jct. Sanson St. - National Highway) Westernside, Brgy. San Francisco",
            "code": "DRRMF22-1000-2-a7",
            "location": "[\"11\"]",
            "cost": 2900816.51,
            "contractor": {
              "data": {
                "id": "9",
                "attributes": {
                  "name": "Johairy Builders",
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
            "start": "2022-10-24",
            "target": "2023-03-03",
            "office": "Engineer's Office",
            "assigned": null,
            "appropriation": 3000000,
            "abc": 2908093.74,
            "status_id": {
              "data": null
            },
            "remarks": "\"Temporarily stopped due to road right of way problem / FRONTING cathedral\n\nCurrently, has resumed work as of 9/26/2024\n\n80% accomplishment as of Dec.6, 2024\n",
            "entry_type": "IMPLEMENTED",
            "bid_date": "2022-08-15",
            "year": 2022,
            "abc_published": null,
            "prop_takers": null,
            "prop_infra": null,
            "time_extension": null
          }
        }), db_connection).await,
        convert_projects_into_active_model_v2(&json!({
          "id": "29",
          "attributes": {
            "project_status": "Completed",
            "name": "Construction of Sidewalk and Underground Drainage Along Ariosa St. (Jct.Sagun St.- National Highway)Eastern Side, Brgy. San Francisco",
            "code": "DRRMF22-1000-2-a8",
            "location": "[\"11\"]",
            "cost": 4354465.66,
            "contractor": {
              "data": {
                "id": "7",
                "attributes": {
                  "name": "St. Joseph Construction",
                  "street_purok": null
                }
              }
            },
            "duration": 140,
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
            "start": "2023-02-16",
            "target": "2023-07-06",
            "office": "Engineer's Office",
            "assigned": null,
            "appropriation": 4500000,
            "abc": 4358389.08,
            "status_id": {
              "data": null
            },
            "remarks": "\"* 100% physically completed based on approved technical plans & specifications\n* w/ time extension as per approved SO No.1\n* Adjusted Targe Date of Completion: Sept. 6, 2023\"\n* Actual Date Completed: 9/6/2023",
            "entry_type": "IMPLEMENTED",
            "bid_date": "2022-12-20",
            "year": 2022,
            "abc_published": null,
            "prop_takers": "\"CHAMPION IPIL- ELENA-09278305516\"",
            "prop_infra": null,
            "time_extension": null
          }
        }), db_connection).await,
        convert_projects_into_active_model_v2(&json!({
          "id": "30",
          "attributes": {
            "project_status": "On-Going",
            "name": "Construction of Sidewalk and Underground Drainage Along Alano St. (Jct.Sanson St.- National Highway) Western Side",
            "code": "DRRMF22-1000-2-a9",
            "location": "[]",
            "cost": 2906548.95,
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
            "appropriation": 3000000,
            "abc": 0,
            "status_id": {
              "data": null
            },
            "remarks": null,
            "entry_type": "IMPLEMENTED",
            "bid_date": "2022-12-27",
            "year": 2022,
            "abc_published": null,
            "prop_takers": null,
            "prop_infra": null,
            "time_extension": null
          }
        }), db_connection).await,
convert_projects_into_active_model_v2(&json!({
          "id": "31",
          "attributes": {
            "project_status": "Completed",
            "name": "Construction of Sidewalk and Underground Drainage Along Alano St. (Jct.Sagun St.- National Highway)Eastern Side",
            "code": "DRRMF22-1000-2-a10",
            "location": "[]",
            "cost": 4357889.08,
            "contractor": {
              "data": {
                "id": "1",
                "attributes": {
                  "name": "CTG Construction",
                  "street_purok": "Purok 1"
                }
              }
            },
            "duration": 140,
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
            "start": "2023-03-01",
            "target": "2023-07-19",
            "office": "Engineer's Office",
            "assigned": null,
            "appropriation": 4500000,
            "abc": 0,
            "status_id": {
              "data": null
            },
            "remarks": "\"* 100% physically completed based on approved technical plans & specifications\n* w/ time extensio due to SO No. 1\n* Adjusted Target Date of Completion: Oct. 1, 2023\"\n* Actual date Completed : 9/19/2023\n",
            "entry_type": "IMPLEMENTED",
            "bid_date": "2022-12-20",
            "year": 2022,
            "abc_published": null,
            "prop_takers": "\" JABO (LANCE CO)\"",
            "prop_infra": null,
            "time_extension": null
          }
        }), db_connection).await,
convert_projects_into_active_model_v2(&json!({
          "id": "32",
          "attributes": {
            "project_status": "Completed",
            "name": "Construction of Sidewalk and Underground Drainage Along Aquino St. (Jct. Jamisola St. - R. Magsaysay St.)Bothsides",
            "code": "DRRMF22-1000-2-a13",
            "location": "[]",
            "cost": 2905469.71,
            "contractor": {
              "data": {
                "id": "8",
                "attributes": {
                  "name": "JAMT Construction",
                  "street_purok": null
                }
              }
            },
            "duration": 140,
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
            "start": "2023-03-03",
            "target": "2023-07-21",
            "office": "Engineer's Office",
            "assigned": null,
            "appropriation": 3000000,
            "abc": 0,
            "status_id": {
              "data": null
            },
            "remarks": "\"* 100% physically completed based on approved technical plans & specifications\n* w/ time extension due to VO No.1\n* Adjusted Target of Completion Date: Nov.  23, 2023\"\n* Actual Date Completed:11/17/2023",
            "entry_type": "IMPLEMENTED",
            "bid_date": "2022-12-27",
            "year": 2022,
            "abc_published": null,
            "prop_takers": "ARIES MADARANG",
            "prop_infra": null,
            "time_extension": null
          }
        }), db_connection).await,
convert_projects_into_active_model_v2(&json!({
          "id": "33",
          "attributes": {
            "project_status": "Completed",
            "name": "Construction of Sidewalk and Underground Drainage Along R. Magsaysay Ave. (Jct. Datoc St. - F.S Pajares Ave.) Northern Side",
            "code": " DRRMF22-1000-2-a14",
            "location": "[]",
            "cost": 2882276.3,
            "contractor": {
              "data": {
                "id": "5",
                "attributes": {
                  "name": "MOLROW CONSTRUCTION",
                  "street_purok": null
                }
              }
            },
            "duration": 120,
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
            "start": "2023-05-15",
            "target": "2023-09-12",
            "office": "Engineer's Office",
            "assigned": null,
            "appropriation": 3000000,
            "abc": 0,
            "status_id": {
              "data": null
            },
            "remarks": "\"* 100% physically completed based on approved technical plans & specifications\n* w/ time extension as per approved VO No.1 \n* Adjusted Targe Date of Completion: Oct. 15, 2023\"\n* Actual Date Completed: 10/12/2023",
            "entry_type": "IMPLEMENTED",
            "bid_date": "2023-03-29",
            "year": 2022,
            "abc_published": null,
            "prop_takers": "SADAM DIMASAR",
            "prop_infra": null,
            "time_extension": null
          }
        }), db_connection).await,
convert_projects_into_active_model_v2(&json!({
          "id": "34",
          "attributes": {
            "project_status": "On-Going",
            "name": "Construction of Sidewalk and Underground Drainage Along R. Magsaysay Ave. (from San Pedro Bridge- Kawit Bridge)Left Side",
            "code": "DRRMF22-1000-2-a15",
            "location": "[\"8\"]",
            "cost": 3853000,
            "contractor": {
              "data": {
                "id": "1",
                "attributes": {
                  "name": "CTG Construction",
                  "street_purok": "Purok 1"
                }
              }
            },
            "duration": 130,
            "adjusted": null,
            "total": null,
            "weight": null,
            "prop_sdg": "[]",
            "prop_fund": "3",
            "prop_type": "7",
            "prop_category": "1",
            "prop_sector": "[]",
            "prop_assign": "2",
            "accom_total": 48,
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
            "start": "2022-11-20",
            "target": "2023-03-30",
            "office": "Engineer's Office",
            "assigned": null,
            "appropriation": 4000000,
            "abc": 3863387.6,
            "status_id": {
              "data": null
            },
            "remarks": "On-going implementation of the project, delayed completion of the project due to site problem\n* with 48% physical accomplishment as of Dec. 6, 2024",
            "entry_type": "IMPLEMENTED",
            "bid_date": "2022-11-10",
            "year": 2022,
            "abc_published": null,
            "prop_takers": "RD VINCE-DOLE",
            "prop_infra": null,
            "time_extension": null
          }
        }), db_connection).await,
convert_projects_into_active_model_v2(&json!({
          "id": "35",
          "attributes": {
            "project_status": "Completed",
            "name": "Impvt of Banana Tissue Laboratory Building (Macasing) - (GAD22-8000-A2)",
            "code": "GAD22-8000-A2/joint fund w/DF23-8000-B2m",
            "location": "[\"44\"]",
            "cost": 967125.98,
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
            "start": "2023-01-16",
            "target": "2023-12-15",
            "office": "Engineer's Office",
            "assigned": null,
            "appropriation": 1000000,
            "abc": 969261.17,
            "status_id": {
              "data": null
            },
            "remarks": "Joint funding from GAD DF23-8000-B2m and GAD22-8000-A2...\n\n\"* 100% physically completed base on approved technical plans & specifications  as of Dec. 15, 2023\n* w/ time extension as per approved Suspension Order No. 1\n* Adjusted Targe Date of Completion: Dec. 16, 2023\"\n",
            "entry_type": "IMPLEMENTED",
            "bid_date": "2022-11-03",
            "year": 2022,
            "abc_published": null,
            "prop_takers": "CAPT. DACAL",
            "prop_infra": null,
            "time_extension": 243
          }
        }), db_connection).await,
convert_projects_into_active_model_v2(&json!({
          "id": "36",
          "attributes": {
            "project_status": "Not Yet Started",
            "name": "Const/Impvt of Gawad Kalinga - Nazareth Road. Buenavista",
            "code": "1SAIP22 - 8000R - C1a1",
            "location": "[]",
            "cost": 675620.47,
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
            "prop_sdg": "[]",
            "prop_fund": "5",
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
            "appropriation": 700000,
            "abc": 0,
            "status_id": {
              "data": null
            },
            "remarks": null,
            "entry_type": "IMPLEMENTED",
            "bid_date": "2022-11-03",
            "year": 2022,
            "abc_published": null,
            "prop_takers": null,
            "prop_infra": null,
            "time_extension": null
          }
        }), db_connection).await,
convert_projects_into_active_model_v2(&json!({
          "id": "37",
          "attributes": {
            "project_status": "Completed",
            "name": "Concreting of Road at Barangay Balintawak (Frm Purok Sili To Palpalan Boundary)",
            "code": "1SAIP22 - 8000R - C1a2",
            "location": null,
            "cost": 769440,
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
            "appropriation": 800000,
            "abc": 0,
            "status_id": {
              "data": null
            },
            "remarks": null,
            "entry_type": "IMPLEMENTED",
            "bid_date": "2022-12-12",
            "year": 2022,
            "abc_published": null,
            "prop_takers": null,
            "prop_infra": null,
            "time_extension": null
          }
        }), db_connection).await,
convert_projects_into_active_model_v2(&json!({
          "id": "38",
          "attributes": {
            "project_status": "Completed",
            "name": "Improvement/Concreting of Parking Area at Rotunda (Phase III)\n",
            "code": "DF22-8000-B1i",
            "location": "[]",
            "cost": 2879511.14,
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
            "start": "Invalid date",
            "target": "Invalid date",
            "office": "Engineer's Office",
            "assigned": null,
            "appropriation": 3000000,
            "abc": 0,
            "status_id": {
              "data": null
            },
            "remarks": null,
            "entry_type": "IMPLEMENTED",
            "bid_date": "2022-12-27",
            "year": 2022,
            "abc_published": null,
            "prop_takers": null,
            "prop_infra": null,
            "time_extension": null
          }
        }), db_connection).await,
convert_projects_into_active_model_v2(&json!({
          "id": "39",
          "attributes": {
            "project_status": "Completed",
            "name": "Widening of Danlugan Barangay Road (Leading to Cemetery) Right Side\n",
            "code": "DF22-8000-B1j",
            "location": "[]",
            "cost": 1927139.19,
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
            "prop_fund": "2",
            "prop_type": null,
            "prop_category": "",
            "prop_sector": "[]",
            "prop_assign": "",
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
            "start": "Invalid date",
            "target": "Invalid date",
            "office": "Engineer's Office",
            "assigned": null,
            "appropriation": 2000000,
            "abc": 0,
            "status_id": {
              "data": null
            },
            "remarks": null,
            "entry_type": "IMPLEMENTED",
            "bid_date": "2022-03-23",
            "year": 2022,
            "abc_published": null,
            "prop_takers": null,
            "prop_infra": null,
            "time_extension": null
          }
        }), db_connection).await,
        convert_projects_into_active_model_v2(&json!({
              "id": "40",
              "attributes": {
                "project_status": "On-Going",
                "name": "Widening/Concreting of Road at Rotunda Hills",
                "code": "DF22-8000-B1k",
                "location": "[]",
                "cost": 4830001.05,
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
                "prop_fund": "2",
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
                "start": "Invalid date",
                "target": "Invalid date",
                "office": "Engineer's Office",
                "assigned": null,
                "appropriation": 5000000,
                "abc": 0,
                "status_id": {
                  "data": null
                },
                "remarks": null,
                "entry_type": "IMPLEMENTED",
                "bid_date": "2023-01-17",
                "year": 2022,
                "abc_published": null,
                "prop_takers": null,
                "prop_infra": null,
                "time_extension": null
              }
            }), db_connection).await,
    convert_projects_into_active_model_v2(&json!({
              "id": "41",
              "attributes": {
                 "project_status": "Completed",
                "name": "Construction of Bleacher and Stage at Covered Court PAGSCI",
                "code": "1SAIP22 - 8000R - C1b2",
                "location": "[]",
                "cost": 951485.16,
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
                "prop_fund": "5",
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
                "appropriation": 1000000,
                "abc": 0,
                "status_id": {
                  "data": null
                },
                "remarks": null,
                "entry_type": "IMPLEMENTED",
                "bid_date": "2023-01-23",
                "year": 2022,
                "abc_published": null,
                "prop_takers": null,
                "prop_infra": null,
                "time_extension": null
              }
            }), db_connection).await,
    convert_projects_into_active_model_v2(&json!({
              "id": "42",
              "attributes": {
                "project_status": "Not Yet Started",
                "name": "Concreting of Road at Danlugan Agri Farm (Phase II)",
                "code": "DF22-8000-B1m",
                "location": "[\"27\"]",
                "cost": 2898703.07,
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
                "prop_fund": "2",
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
                "start": "",
                "target": null,
                "office": "Engineer's Office",
                "assigned": null,
                "appropriation": 3000000,
                "abc": 2900590.12,
                "status_id": {
                  "data": null
                },
                "remarks": "ON-HOLD  due to ongoing site preparation undertaken by ADMIN\n",
                "entry_type": "IMPLEMENTED",
                "bid_date": "2023-12-27",
                "year": 2022,
                "abc_published": null,
                "prop_takers": "CAPT. DACAL",
                "prop_infra": null,
                "time_extension": null
              }
            }), db_connection).await,
    convert_projects_into_active_model_v2(&json!({
              "id": "43",
              "attributes": {
                "project_status": "Completed",
                "name": "Insatallation of Water Connection to Purok Mangga",
                "code": "1SAIP22 - 8000R - C1c1",
                "location": "[]",
                "cost": 385472.31,
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
                "prop_type": "9",
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
                "appropriation": 400000,
                "abc": 0,
                "status_id": {
                  "data": null
                },
                "remarks": null,
                "entry_type": "IMPLEMENTED",
                "bid_date": "2023-02-27",
                "year": 2022,
                "abc_published": null,
                "prop_takers": null,
                "prop_infra": null,
                "time_extension": null
              }
            }), db_connection).await,
    convert_projects_into_active_model_v2(&json!({
              "id": "44",
              "attributes": {
                "project_status": "Not Yet Started",
                "name": "Const of Pathway at Zone 1 & 2 (Near E/S) Muricay",
                "code": "1SAIP22 - 8000R - A1E6",
                "location": "[]",
                "cost": 290210.9,
                "contractor": {
                  "data": {
                    "id": "14",
                    "attributes": {
                      "name": "Ammara Strukture Construction",
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
                "bid_date": "2022-11-03",
                "year": 2022,
                "abc_published": null,
                "prop_takers": null,
                "prop_infra": null,
                "time_extension": null
              }
            }), db_connection).await,
    convert_projects_into_active_model_v2(&json!({
              "id": "45",
              "attributes": {
                "project_status": "Not Yet Started",
                "name": "Construction of Sidewalk & Underground Drainage along Dablo St. \n",
                "code": "DF22-8000-B2-c",
                "location": "[]",
                "cost": 2877157.93,
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
                "appropriation": 3000000,
                "abc": 0,
                "status_id": {
                  "data": null
                },
                "remarks": null,
                "entry_type": "IMPLEMENTED",
                "bid_date": "2022-12-14",
                "year": 2022,
                "abc_published": null,
                "prop_takers": null,
                "prop_infra": null,
                "time_extension": null
              }
            }), db_connection).await,
    convert_projects_into_active_model_v2(&json!({
              "id": "46",
              "attributes": {
                "project_status": "Completed",
                "name": "Concreting of Datagan-Kendis Cave Barangay Road Phase II",
                "code": "3SAIP22 - 8000R - B1a",
                "location": "[]",
                "cost": 14483158.82,
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
                "prop_fund": "7",
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
                "appropriation": 15000000,
                "abc": 0,
                "status_id": {
                  "data": null
                },
                "remarks": null,
                "entry_type": "IMPLEMENTED",
                "bid_date": "2022-09-19",
                "year": 2022,
                "abc_published": null,
                "prop_takers": null,
                "prop_infra": null,
                "time_extension": null
              }
            }), db_connection).await,
    convert_projects_into_active_model_v2(&json!({
              "id": "47",
              "attributes": {
                "project_status": "Completed",
                "name": "Improvement of Boulevard Entrance (near Wharf Areas) and Fishport (Including Drainage/RCCP)",
                "code": "DF22-8000-B2-e",
                "location": "[\"13\"]",
                "cost": 968915.9,
                "contractor": {
                  "data": {
                    "id": "16",
                    "attributes": {
                      "name": "AJT Construction",
                      "street_purok": null
                    }
                  }
                },
                "duration": 75,
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
                "start": "2023-01-16",
                "target": "2023-04-01",
                "office": "Engineer's Office",
                "assigned": null,
                "appropriation": 1000000,
                "abc": 969958.41,
                "status_id": {
                  "data": null
                },
                "remarks": "100% physically completed based on approved technical plans & specifications\n",
                "entry_type": "IMPLEMENTED",
                "bid_date": "2022-12-12",
                "year": 2022,
                "abc_published": null,
                "prop_takers": "King Macaampao",
                "prop_infra": null,
                "time_extension": null
              }
            }), db_connection).await,
    convert_projects_into_active_model_v2(&json!({
              "id": "48",
              "attributes": {
                "project_status": "On-Going",
                "name": "Completion of Underground Drainage with Sidewalk from Sto Nino- Banale (Boundary (Bothsides)\n",
                "code": "DF22-8000-B2-f",
                "location": "[\"3\"]",
                "cost": 1923505.8,
                "contractor": {
                  "data": {
                    "id": "3",
                    "attributes": {
                      "name": "GAF TRADING & CONST",
                      "street_purok": null
                    }
                  }
                },
                "duration": 130,
                "adjusted": null,
                "total": null,
                "weight": null,
                "prop_sdg": "[]",
                "prop_fund": "2",
                "prop_type": "7",
                "prop_category": "1",
                "prop_sector": "[]",
                "prop_assign": "2",
                "accom_total": 84,
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
                "start": "2022-12-01",
                "target": "2023-04-10",
                "office": "Engineer's Office",
                "assigned": null,
                "appropriation": 2000000,
                "abc": 1924305.65,
                "status_id": {
                  "data": null
                },
                "remarks": "ON-GOING but NON-MOVING\n",
                "entry_type": "IMPLEMENTED",
                "bid_date": "2022-09-28",
                "year": 2022,
                "abc_published": null,
                "prop_takers": null,
                "prop_infra": null,
                "time_extension": null
              }
            }), db_connection).await,
    convert_projects_into_active_model_v2(&json!({
              "id": "49",
              "attributes": {
                "project_status": "Completed",
                "name": "Construction of Sidewalk & Underground Drainage Along Fernan St.(Western Side) Completion\n",
                "code": "DF22-8000-B2-g",
                "location": null,
                "cost": 574867.8,
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
                "prop_sdg": null,
                "prop_fund": "2",
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
                "appropriation": 600000,
                "abc": 0,
                "status_id": {
                  "data": null
                },
                "remarks": null,
                "entry_type": "IMPLEMENTED",
                "bid_date": "2022-09-30",
                "year": 2022,
                "abc_published": null,
                "prop_takers": null,
                "prop_infra": null,
                "time_extension": null
              }
            }), db_connection).await,
            convert_projects_into_active_model_v2(&json!({
              "id": "50",
              "attributes": {
                 "project_status": "Completed",
                "name": "Widening/Rehab of Road at Brgy Danlugan Leading to Public Cemetery",
                "code": "4SAIP22-8000R-B1-1a1",
                "location": "[]",
                "cost": 3411436.83,
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
                "prop_fund": "8",
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
                "appropriation": 3555000,
                "abc": 0,
                "status_id": {
                  "data": null
                },
                "remarks": null,
                "entry_type": "IMPLEMENTED",
                "bid_date": "2023-01-17",
                "year": 2022,
                "abc_published": null,
                "prop_takers": null,
                "prop_infra": null,
                "time_extension": null
              }
            }), db_connection).await,
    convert_projects_into_active_model_v2(&json!({
              "id": "51",
              "attributes": {
                "project_status": "Completed",
                "name": "Construction of Sidewalk &Underground Drainage Along National Highway (Balangasan Bridge-Sabellano St.) Bothsides\n",
                "code": "DF22-8000-B2-i",
                "location": "[\"1\"]",
                "cost": 7749809.95,
                "contractor": {
                  "data": {
                    "id": "7",
                    "attributes": {
                      "name": "St. Joseph Construction",
                      "street_purok": null
                    }
                  }
                },
                "duration": 140,
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
                "start": "2023-05-29",
                "target": "2023-10-16",
                "office": "Engineer's Office",
                "assigned": null,
                "appropriation": 8000000,
                "abc": 0,
                "status_id": {
                  "data": null
                },
                "remarks": "* 100% physically completed base on approved technical plans & specifications\n\n",
                "entry_type": "IMPLEMENTED",
                "bid_date": "2023-03-29",
                "year": 2022,
                "abc_published": null,
                "prop_takers": "CHAMPION IPIL",
                "prop_infra": null,
                "time_extension": null
              }
            }), db_connection).await,
    convert_projects_into_active_model_v2(&json!({
              "id": "52",
              "attributes": {
                "project_status": "Bidded",
                "name": "Rehab and Restoration of Existing Barangay Road with Construction of Open Draingae and Slope Protection at Barangay Baloyboan",
                "code": "1SAIL22 - 8000 - A1a",
                "location": null,
                "cost": 2199990.25,
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
                "prop_sdg": null,
                "prop_fund": null,
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
                "appropriation": 2300000,
                "abc": 0,
                "status_id": {
                  "data": null
                },
                "remarks": null,
                "entry_type": "IMPLEMENTED",
                "bid_date": "2023-01-30",
                "year": 2022,
                "abc_published": null,
                "prop_takers": null,
                "prop_infra": null,
                "time_extension": null
              }
            }), db_connection).await,
    convert_projects_into_active_model_v2(&json!({
              "id": "53",
              "attributes": {
                "project_status": "Completed",
                "name": "Construction of Sidewalk & Underground Drainage  at Purok Tugas (Jct.National Highway- OLT College) Right Side\n",
                "code": "DF22-8000-B2-j",
                "location": "[\"1\"]",
                "cost": 3863935.43,
                "contractor": {
                  "data": {
                    "id": "17",
                    "attributes": {
                      "name": "Victer Construction & Devt, OPC",
                      "street_purok": null
                    }
                  }
                },
                "duration": 140,
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
                "start": "2023-03-20",
                "target": "2023-08-07",
                "office": "Engineer's Office",
                "assigned": null,
                "appropriation": 4000000,
                "abc": 3869235.41,
                "status_id": {
                  "data": null
                },
                "remarks": "\"* 100% physically completed based on approved technical plans & specifications\n* w/ time extension as per approved SO No.1\n* Adjusted Targe Date of Completion: Nov. 22, 2023\"\n",
                "entry_type": "IMPLEMENTED",
                "bid_date": "2023-01-17",
                "year": 2022,
                "abc_published": null,
                "prop_takers": null,
                "prop_infra": null,
                "time_extension": null
              }
            }), db_connection).await,
    convert_projects_into_active_model_v2(&json!({
              "id": "54",
              "attributes": {
                "project_status": "On-Going",
                "name": "Construction of Sidewalk & Underground Drainage Along Cabrera St. (Jct.FS Pajares St.-Aquino)Southernside\n",
                "code": "DF22-8000-B2-l",
                "location": "[]",
                "cost": 1916221.08,
                "contractor": {
                  "data": {
                    "id": "18",
                    "attributes": {
                      "name": "Domphil Construction",
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
                "appropriation": 2000000,
                "abc": 0,
                "status_id": {
                  "data": null
                },
                "remarks": null,
                "entry_type": "IMPLEMENTED",
                "bid_date": "2023-01-30",
                "year": 2022,
                "abc_published": null,
                "prop_takers": null,
                "prop_infra": null,
                "time_extension": null
              }
            }), db_connection).await,
    convert_projects_into_active_model_v2(&json!({
              "id": "55",
              "attributes": {
                "project_status": "Completed",
                "name": "Sidewalkat National Highway (Crossing Tiguma leading to Petron) Southernside\n",
                "code": "DF22-8000-B2-m",
                "location": "[\"18\"]",
                "cost": 2860546.56,
                "contractor": {
                  "data": {
                    "id": "19",
                    "attributes": {
                      "name": "Jenrich Construction",
                      "street_purok": null
                    }
                  }
                },
                "duration": 140,
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
                "start": "2023-02-03",
                "target": "2024-01-04",
                "office": "Engineer's Office",
                "assigned": null,
                "appropriation": 3000000,
                "abc": 0,
                "status_id": {
                  "data": null
                },
                "remarks": "100% PHYSICALLY COMPLETED w/ time extension as per approved Suspension Order No.1",
                "entry_type": "IMPLEMENTED",
                "bid_date": "2022-12-22",
                "year": 2022,
                "abc_published": null,
                "prop_takers": "COUN CAGAMPANG",
                "prop_infra": null,
                "time_extension": 195
              }
            }), db_connection).await,
    convert_projects_into_active_model_v2(&json!({
              "id": "56",
              "attributes": {
                "project_status": "On-Going",
                "name": "Impvt./Rehab. Of Sidewalk Along Aquino St. (Jct.Salazar St.-Urro St.)Bothsides\n",
                "code": "DF22-8000-B2-o",
                "location": "[]",
                "cost": 1453986.21,
                "contractor": {
                  "data": {
                    "id": "20",
                    "attributes": {
                      "name": "Maqueserg Construction",
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
                "prop_type": "1",
                "prop_category": "4",
                "prop_sector": "[]",
                "prop_assign": "2",
                "accom_total": 79,
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
                "remarks": "",
                "entry_type": "IMPLEMENTED",
                "bid_date": "2022-12-14",
                "year": 2022,
                "abc_published": null,
                "prop_takers": null,
                "prop_infra": null,
                "time_extension": null
              }
            }), db_connection).await,
    convert_projects_into_active_model_v2(&json!({
              "id": "57",
              "attributes": {
                "project_status": "Completed",
                "name": "Impvt./Rehab. Of Sidewalk Along Datoc St. (Jct. Sagun St.- National Highway) Bothsides\n",
                "code": "DF22-8000-B2-p",
                "location": "[]",
                "cost": 2880375.56,
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
                "appropriation": 3000000,
                "abc": 0,
                "status_id": {
                  "data": null
                },
                "remarks": null,
                "entry_type": "IMPLEMENTED",
                "bid_date": "2022-12-27",
                "year": 2022,
                "abc_published": null,
                "prop_takers": null,
                "prop_infra": null,
                "time_extension": null
              }
            }), db_connection).await,
    convert_projects_into_active_model_v2(&json!({
              "id": "58",
              "attributes": {
                "project_status": "Completed",
                "name": "Construction of Danlugan Wet Market\n",
                "code": "DF22-8000-B4-b",
                "location": "[\"27\"]",
                "cost": 2403930.82,
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
                "prop_fund": "2",
                "prop_type": "3",
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
                "remarks": "Completed",
                "entry_type": "IMPLEMENTED",
                "bid_date": "2022-09-28",
                "year": 2022,
                "abc_published": null,
                "prop_takers": null,
                "prop_infra": null,
                "time_extension": null
              }
            }), db_connection).await,
    convert_projects_into_active_model_v2(&json!({
              "id": "59",
              "attributes": {
                "project_status": "Completed",
                "name": "Construction of Dog Shelter Building at Old Dumpsite, Bagong Silang",
                "code": "DF22-8000-B4-c",
                "location": "[\"1\"]",
                "cost": 488003.69,
                "contractor": {
                  "data": {
                    "id": "2",
                    "attributes": {
                      "name": "ALZ Construction",
                      "street_purok": null
                    }
                  }
                },
                "duration": 60,
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
                "start": "2024-07-01",
                "target": "2024-08-30",
                "office": "Engineer's Office",
                "assigned": null,
                "appropriation": 500000,
                "abc": 488014.67,
                "status_id": {
                  "data": null
                },
                "remarks": "* 100% physically completed based on approved plans & specifications \n\n* date Completed: 8/20/2024",
                "entry_type": "IMPLEMENTED",
                "bid_date": "2022-09-28",
                "year": 2022,
                "abc_published": null,
                "prop_takers": "IRENE",
                "prop_infra": null,
                "time_extension": null
              }
            }), db_connection).await,
convert_projects_into_active_model_v2(&json!({
              "id": "60",
              "attributes": {
                "project_status": "On-Going",
                "name": "Construction of Perimeter Fence at Pagadian Beach Resort at Poloyagan",
                "code": "DF22-8000-B5-a",
                "location": "[\"49\"]",
                "cost": 2876817.31,
                "contractor": {
                  "data": {
                    "id": "17",
                    "attributes": {
                      "name": "Victer Construction & Devt, OPC",
                      "street_purok": null
                    }
                  }
                },
                "duration": 150,
                "adjusted": null,
                "total": null,
                "weight": null,
                "prop_sdg": "[]",
                "prop_fund": "2",
                "prop_type": "28",
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
                "start": "2023-08-20",
                "target": "2024-01-17",
                "office": "Engineer's Office",
                "assigned": null,
                "appropriation": 3000000,
                "abc": 2876817.31,
                "status_id": {
                  "data": null
                },
                "remarks": "On-going implementation of the project\n",
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
              "id": "61",
              "attributes": {
                "project_status": "On-Going",
                "name": "Materials Laboratory\n",
                "code": "RF22-1000-1-13-c",
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
                "remarks": "Implemented By Admin",
                "entry_type": "IMPLEMENTED",
                "bid_date": null,
                "year": 2022,
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