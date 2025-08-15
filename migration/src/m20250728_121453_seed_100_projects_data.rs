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
                "id": "476",
                "attributes": {
                  "project_status": "Completed",
                    "name": "Construction/Rehabilitation of Roads/Drainages, River Control, Sea Wall Along Datoc St. (Jct Sagun St. - Sanson St.) Western Side",
                    "code": "DRRMF20 - 1000 - 2 - b16",
                    "location": null,
                    "cost": 1143544.28,
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
                    "prop_fund": "3",
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
                    "appropriation": 1200000,
                    "abc": 0,
                    "status_id": {
                        "data": null
                    },
                    "remarks": null,
                    "entry_type": "IMPLEMENTED",
                    "bid_date": "2020-05-27",
                    "year": 2020,
                    "abc_published": null,
                    "prop_takers": null,
                    "prop_infra": null,
                    "time_extension": null
                }
            }), db_connection).await,
            convert_projects_into_active_model_v2(&json!({
                "id": "477",
                "attributes": {
                  "project_status": "Completed",
                    "name": "Construction/Rehabilitation of Roads/Drainages, River Control, Sea Wall Along Urro St. (Jct Zulueta St - RT Lim St) Northern Side",
                    "code": "DRRMF20 - 1000 - 2 - b17",
                    "location": null,
                    "cost": 1151635.5,
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
                    "prop_fund": "3",
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
                    "appropriation": 1200000,
                    "abc": 0,
                    "status_id": {
                        "data": null
                    },
                    "remarks": null,
                    "entry_type": "IMPLEMENTED",
                    "bid_date": "2020-04-20",
                    "year": 2020,
                    "abc_published": null,
                    "prop_takers": null,
                    "prop_infra": null,
                    "time_extension": null
                }
            }), db_connection).await,
            convert_projects_into_active_model_v2(&json!({
                "id": "478",
                "attributes": {
                   "project_status": "Completed",
                    "name": "Construction/Rehabilitation of Roads/Drainages, River Control, Sea Wall Along Urro St. (Jct Zulueta St - RT Lim St) Southern Side",
                    "code": "DRRMF20 - 1000 - 2 - b18",
                    "location": null,
                    "cost": 1151898.81,
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
                    "prop_fund": "3",
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
                    "appropriation": 1200000,
                    "abc": 0,
                    "status_id": {
                        "data": null
                    },
                    "remarks": null,
                    "entry_type": "IMPLEMENTED",
                    "bid_date": "2020-05-26",
                    "year": 2020,
                    "abc_published": null,
                    "prop_takers": null,
                    "prop_infra": null,
                    "time_extension": null
                }
            }), db_connection).await,
            convert_projects_into_active_model_v2(&json!({
                "id": "479",
                "attributes": {
                  "project_status": "Completed",
                    "name": "Construction/Rehabilitation of Roads/Drainages, River Control, Sea Wall Along Urro St. (Jct Datoc St - Bridge) Southern Side",
                    "code": "DRRMF20 - 1000 - 2 - b19",
                    "location": null,
                    "cost": 1114634.75,
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
                    "prop_fund": "3",
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
                    "appropriation": 1150000,
                    "abc": 0,
                    "status_id": {
                        "data": null
                    },
                    "remarks": null,
                    "entry_type": "IMPLEMENTED",
                    "bid_date": "2020-05-27",
                    "year": 2020,
                    "abc_published": null,
                    "prop_takers": null,
                    "prop_infra": null,
                    "time_extension": null
                }
            }), db_connection).await,
            convert_projects_into_active_model_v2(&json!({
    "id": "480",
    "attributes": {
      "project_status": "Bidded",
        "name": "Construction/Rehabilitation of Roads/Drainages, River Control, Sea Wall Along Urro St. (Jct Sabellano St - Creek) Southern Side",
        "code": "DRRMF20 - 1000 - 2 - b20",
        "location": null,
        "cost": 1156452.5,
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
        "prop_sdg": null,
        "prop_fund": "3",
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
        "appropriation": 1200000,
        "abc": 0,
        "status_id": {
            "data": null
        },
        "remarks": null,
        "entry_type": "IMPLEMENTED",
        "bid_date": "2021-06-08",
        "year": 2020,
        "abc_published": null,
        "prop_takers": null,
        "prop_infra": null,
        "time_extension": null
    }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
    "id": "481",
    "attributes": {
      "project_status": "Completed",
        "name": "Along Urro St. (Jct Roxas - Sabellano St) Northern Side",
        "code": "DRRMF20 - 1000 - 2 - b21",
        "location": null,
        "cost": 1148011,
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
        "prop_fund": "3",
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
        "appropriation": 1200000,
        "abc": 0,
        "status_id": {
            "data": null
        },
        "remarks": null,
        "entry_type": "IMPLEMENTED",
        "bid_date": "2020-06-03",
        "year": 2020,
        "abc_published": null,
        "prop_takers": null,
        "prop_infra": null,
        "time_extension": null
    }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
    "id": "482",
    "attributes": {
      "project_status": "Completed",
        "name": "Construction/Rehabilitation of Roads/Drainages, River Control, Sea Wall Along Jamisola St. (Jct Bonifacio St - Alano St) Southern Side",
        "code": "DRRMF20 - 1000 - 2 - b22",
        "location": "[]",
        "cost": 1186925.22,
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
        "bid_date": "2021-02-02",
        "year": 2020,
        "abc_published": null,
        "prop_takers": null,
        "prop_infra": null,
        "time_extension": null
    }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
    "id": "483",
    "attributes": {
      "project_status": "Completed",
        "name": "Construction/Rehabilitation of Roads/Drainages, River Control, Sea Wall Along Jamisola St. (Jct FS PAJARES. - Alano St) Northern Side",
        "code": "DRRMF20 - 1000 - 2 - b23",
        "location": null,
        "cost": 1186925.22,
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
        "prop_sdg": null,
        "prop_fund": "3",
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
        "appropriation": 1200000,
        "abc": 0,
        "status_id": {
            "data": null
        },
        "remarks": null,
        "entry_type": "IMPLEMENTED",
        "bid_date": "2020-01-19",
        "year": 2020,
        "abc_published": null,
        "prop_takers": null,
        "prop_infra": null,
        "time_extension": null
    }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
    "id": "484",
    "attributes": {
      "project_status": "Completed",
        "name": "Construction/Rehabilitation of Roads/Drainages, River Control, Sea Wall Along R. Magsaysay St (Jct FS Pajares St - Alano St) Northern Side",
        "code": "DRRMF20 - 1000 - 2 - b24",
        "location": null,
        "cost": 2292936.08,
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
        "prop_sdg": null,
        "prop_fund": "3",
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
        "appropriation": 2400000,
        "abc": 0,
        "status_id": {
            "data": null
        },
        "remarks": null,
        "entry_type": "IMPLEMENTED",
        "bid_date": "2020-04-20",
        "year": 2020,
        "abc_published": null,
        "prop_takers": null,
        "prop_infra": null,
        "time_extension": null
    }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
    "id": "485",
    "attributes": {
      "project_status": "Completed",
        "name": "Construction/Rehabilitation of Roads/Drainages, River Control, Sea Wall Along R. Magsaysay St (Jct FS Pajares St - Alano St) Southern Side",
        "code": "DRRMF20 - 1000 - 2 - b25",
        "location": "[]",
        "cost": 1247466.01,
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
        "appropriation": 1250000,
        "abc": 0,
        "status_id": {
            "data": null
        },
        "remarks": null,
        "entry_type": "IMPLEMENTED",
        "bid_date": "2020-06-03",
        "year": 2020,
        "abc_published": null,
        "prop_takers": null,
        "prop_infra": null,
        "time_extension": null
    }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
    "id": "486",
    "attributes": {
      "project_status": "Completed",
        "name": "Along R. Magsaysay St (Jct FS Alano St - Fernan St) Southern Side",
        "code": "DRRMF20 - 1000 - 2 - b26",
        "location": null,
        "cost": 2403781.12,
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
        "prop_sdg": null,
        "prop_fund": "3",
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
        "appropriation": 2480000,
        "abc": 0,
        "status_id": {
            "data": null
        },
        "remarks": null,
        "entry_type": "IMPLEMENTED",
        "bid_date": "2020-03-23",
        "year": 2020,
        "abc_published": null,
        "prop_takers": null,
        "prop_infra": null,
        "time_extension": null
    }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
    "id": "487",
    "attributes": {
      "project_status": "Not Yet Started",
        "name": "Construction of New Senior Citizen Bldg",
        "code": "SCPWD20 - 3000 - A3",
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
        "year": 2020,
        "abc_published": null,
        "prop_takers": null,
        "prop_infra": null,
        "time_extension": null
    }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
    "id": "488",
    "attributes": {
      "project_status": "Not Yet Started",
        "name": "Construction of Proposed PWD Office Building",
        "code": "SCPWD20 - 3000 - B3",
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
        "year": 2020,
        "abc_published": null,
        "prop_takers": null,
        "prop_infra": null,
        "time_extension": null
    }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
    "id": "489",
    "attributes": {
      "project_status": "Completed",
        "name": "Const. of Perimeter Fence at Sta. Lucia Elem School",
        "code": "SEF",
        "location": null,
        "cost": 0,
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
        "prop_sdg": null,
        "prop_fund": "11",
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
        "appropriation": 2000000,
        "abc": 0,
        "status_id": {
            "data": null
        },
        "remarks": null,
        "entry_type": "IMPLEMENTED",
        "bid_date": "2021-01-11",
        "year": 2020,
        "abc_published": null,
        "prop_takers": null,
        "prop_infra": null,
        "time_extension": null
    }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
    "id": "490",
    "attributes": {
       "project_status": "Completed",
        "name": "Const/Impvt of Bulatok E/S (Perimeter Fence)",
        "code": "SEF",
        "location": null,
        "cost": 383321.02,
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
        "prop_sdg": null,
        "prop_fund": "11",
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
        "appropriation": 387189,
        "abc": 0,
        "status_id": {
            "data": null
        },
        "remarks": null,
        "entry_type": "IMPLEMENTED",
        "bid_date": "2020-12-08",
        "year": 2020,
        "abc_published": null,
        "prop_takers": null,
        "prop_infra": null,
        "time_extension": null
    }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
    "id": "491",
    "attributes": {
      "project_status": "Completed",
        "name": "Const of City Central Recovery Facility at Composting Area (Phase II)",
        "code": "TRUST FUND",
        "location": null,
        "cost": 553267.37,
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
        "appropriation": 570000,
        "abc": 0,
        "status_id": {
            "data": null
        },
        "remarks": null,
        "entry_type": "IMPLEMENTED",
        "bid_date": "2020-03-06",
        "year": 2020,
        "abc_published": null,
        "prop_takers": null,
        "prop_infra": null,
        "time_extension": null
    }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
    "id": "492",
    "attributes": {
      "project_status": "Completed",
        "name": "Rehabilitation of Banana Tissue Culture Laboratory, Hardening House and Preparation Room - Department of Agriculture Trust FUnd",
        "code": "TRUST FUND",
        "location": null,
        "cost": 479031.89,
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
        "appropriation": 500000,
        "abc": 0,
        "status_id": {
            "data": null
        },
        "remarks": null,
        "entry_type": "IMPLEMENTED",
        "bid_date": "2021-07-06",
        "year": 2020,
        "abc_published": null,
        "prop_takers": null,
        "prop_infra": null,
        "time_extension": null
    }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
    "id": "493",
    "attributes": {
       "project_status": "Not Yet Started",
        "name": "Lot Acquisition/Purchase for Evacuation Center Brgy. Bulatok",
        "code": "1SAIP20 - DR3000 - A2",
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
        "appropriation": 606650,
        "abc": 0,
        "status_id": {
            "data": null
        },
        "remarks": null,
        "entry_type": "IMPLEMENTED",
        "bid_date": null,
        "year": 2020,
        "abc_published": null,
        "prop_takers": null,
        "prop_infra": null,
        "time_extension": null
    }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
    "id": "494",
    "attributes": {
      "project_status": "Not Yet Started",
        "name": "Construction of Residual Contaiment Area including Improvement at Composting Area",
        "code": "1SAIP20 - DR3000 - A1",
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
        "appropriation": 2500000,
        "abc": 0,
        "status_id": {
            "data": null
        },
        "remarks": null,
        "entry_type": "IMPLEMENTED",
        "bid_date": null,
        "year": 2020,
        "abc_published": null,
        "prop_takers": null,
        "prop_infra": null,
        "time_extension": null
    }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
    "id": "495",
    "attributes": {
      "project_status": "On-Going",
        "name": "Construction of Tricycle Terminal Rest Room/Fruitstand at Brgy Buenavista Phase II",
        "code": "1SAIP20 - DR3000 -B1",
        "location": null,
        "cost": 86723.55,
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
        "appropriation": 90000,
        "abc": 0,
        "status_id": {
            "data": null
        },
        "remarks": null,
        "entry_type": "IMPLEMENTED",
        "bid_date": "2020-11-25",
        "year": 2020,
        "abc_published": null,
        "prop_takers": null,
        "prop_infra": null,
        "time_extension": null
    }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
    "id": "496",
    "attributes": {
      "project_status": "Completed",
        "name": "Construction of Additional Liquid Nitrogen 2 (LN2) Building At Barangay Danlugan Phase II",
        "code": "1SAIP20 - DR3000 -B2",
        "location": null,
        "cost": 183659,
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
        "appropriation": 192500,
        "abc": 0,
        "status_id": {
            "data": null
        },
        "remarks": null,
        "entry_type": "IMPLEMENTED",
        "bid_date": "2020-11-25",
        "year": 2020,
        "abc_published": null,
        "prop_takers": null,
        "prop_infra": null,
        "time_extension": null
    }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
    "id": "497",
    "attributes": {
       "project_status": "Completed",
        "name": "Construction of Water Tank Including Pipes and Accessories at City Plaza",
        "code": "1SAIP20 - DR3000 -B3",
        "location": null,
        "cost": 673171.34,
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
        "appropriation": 700000,
        "abc": 0,
        "status_id": {
            "data": null
        },
        "remarks": null,
        "entry_type": "IMPLEMENTED",
        "bid_date": "2020-03-18",
        "year": 2020,
        "abc_published": null,
        "prop_takers": null,
        "prop_infra": null,
        "time_extension": null
    }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
    "id": "498",
    "attributes": {
      "project_status": "Not Yet Started",
        "name": "Construction of Restrooms at Plaza Luz Phase II",
        "code": "1SAIP20 - DR3000 -B4",
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
        "year": 2020,
        "abc_published": null,
        "prop_takers": null,
        "prop_infra": null,
        "time_extension": null
    }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
    "id": "499",
    "attributes": {
      "project_status": "Completed",
        "name": "Construction of Restrooms at Rotunda",
        "code": "\"Joint fund w/: 1SAIP20-DR3000-B5/ 3SAIP21-8000R-C-1c11\"",
        "location": "[\"5\"]",
        "cost": 4040674.6,
        "contractor": {
            "data": {
                "id": "37",
                "attributes": {
                    "name": "ILLANA Bay Construction",
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
        "start": "2022-02-25",
        "target": "2022-07-25",
        "office": "Engineer's Office",
        "assigned": null,
        "appropriation": 2500000,
        "abc": 0,
        "status_id": {
            "data": null
        },
        "remarks": "100%  physically completed based on approved technical plans & specifications\n",
        "entry_type": "IMPLEMENTED",
        "bid_date": "2021-12-06",
        "year": 2020,
        "abc_published": null,
        "prop_takers": "PHILIP DURAN",
        "prop_infra": null,
        "time_extension": null
    }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
    "id": "500",
    "attributes": {
      "project_status": "Completed",
        "name": "Construction of Seawall at Barangay Dumagoc",
        "code": "1SAIP20 - DR3000 -B6",
        "location": null,
        "cost": 963800,
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
        "appropriation": 1000000,
        "abc": 0,
        "status_id": {
            "data": null
        },
        "remarks": null,
        "entry_type": "IMPLEMENTED",
        "bid_date": "2020-03-06",
        "year": 2020,
        "abc_published": null,
        "prop_takers": null,
        "prop_infra": null,
        "time_extension": null
    }
}), db_connection).await,

convert_projects_into_active_model_v2(&json!({
    "id": "501",
    "attributes": {
      "project_status": "Completed",
        "name": "Concreting of Pagadian City Boulevard (PPA Entrance)",
        "code": "1SAIP20 - DR3000 -B7",
        "location": null,
        "cost": 392994.8,
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
        "appropriation": 410000,
        "abc": 0,
        "status_id": {
            "data": null
        },
        "remarks": null,
        "entry_type": "IMPLEMENTED",
        "bid_date": "2020-03-18",
        "year": 2020,
        "abc_published": null,
        "prop_takers": null,
        "prop_infra": null,
        "time_extension": null
    }
}), db_connection).await,

convert_projects_into_active_model_v2(&json!({
    "id": "502",
    "attributes": {
      "project_status": "Completed",
        "name": "Construction of Lechon House (Common Utility), Phase II",
        "code": "1SAIP20 - DR3000 -B8",
        "location": "[]",
        "cost": 693131.18,
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
        "appropriation": 700000,
        "abc": 0,
        "status_id": {
            "data": null
        },
        "remarks": null,
        "entry_type": "IMPLEMENTED",
        "bid_date": "2020-11-04",
        "year": 2020,
        "abc_published": null,
        "prop_takers": null,
        "prop_infra": null,
        "time_extension": null
    }
}), db_connection).await,

convert_projects_into_active_model_v2(&json!({
    "id": "503",
    "attributes": {
      "project_status": "Completed",
        "name": "Construction of Rest Rooms at Plaza Luz (w/ GAD19 - 8000 - B1)",
        "code": "1SAIP20 - RF8000 - B2",
        "location": null,
        "cost": 1939287.71,
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
        "appropriation": 2000000,
        "abc": 0,
        "status_id": {
            "data": null
        },
        "remarks": null,
        "entry_type": "IMPLEMENTED",
        "bid_date": "2020-07-15",
        "year": 2020,
        "abc_published": null,
        "prop_takers": null,
        "prop_infra": null,
        "time_extension": null
    }
}), db_connection).await,

convert_projects_into_active_model_v2(&json!({
    "id": "504",
    "attributes": {
      "project_status": "Completed",
        "name": "Concreting/Improvement of Road Leading to Muslim Village, Brgy. Kawit",
        "code": "2SAIP20 - 8000 - C1a1",
        "location": null,
        "cost": 772272,
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
        "appropriation": 800000,
        "abc": 0,
        "status_id": {
            "data": null
        },
        "remarks": null,
        "entry_type": "IMPLEMENTED",
        "bid_date": "2020-08-05",
        "year": 2020,
        "abc_published": null,
        "prop_takers": null,
        "prop_infra": null,
        "time_extension": null
    }
}), db_connection).await,

convert_projects_into_active_model_v2(&json!({
    "id": "505",
    "attributes": {
      "project_status": "Completed",
        "name": "Concreting/Improvement of Road to Kawit Elementary School, Bgry Kawit",
        "code": "2SAIP20 - 8000 - C1a2",
        "location": null,
        "cost": 482562.98,
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
        "appropriation": 500000,
        "abc": 0,
        "status_id": {
            "data": null
        },
        "remarks": null,
        "entry_type": "IMPLEMENTED",
        "bid_date": "2020-09-08",
        "year": 2020,
        "abc_published": null,
        "prop_takers": null,
        "prop_infra": null,
        "time_extension": null
    }
}), db_connection).await,

convert_projects_into_active_model_v2(&json!({
    "id": "506",
    "attributes": {
      "project_status": "Not Yet Started",
        "name": "Concreting/Improvement of Boulevard Road (Unfinished Portion) Leading to Fishport",
        "code": "2SAIP20 - 8000 - C1a3",
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
        "appropriation": 2000000,
        "abc": 0,
        "status_id": {
            "data": null
        },
        "remarks": null,
        "entry_type": "IMPLEMENTED",
        "bid_date": null,
        "year": 2020,
        "abc_published": null,
        "prop_takers": null,
        "prop_infra": null,
        "time_extension": null
    }
}), db_connection).await,

convert_projects_into_active_model_v2(&json!({
    "id": "507",
    "attributes": {
       "project_status": "Completed",
        "name": "Concreting/Improvement of Road at Sunrise Village, Tiguma",
        "code": "2SAIP20 - 8000 - C1a4",
        "location": null,
        "cost": 1190581.91,
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
        "appropriation": 1250000,
        "abc": 0,
        "status_id": {
            "data": null
        },
        "remarks": null,
        "entry_type": "IMPLEMENTED",
        "bid_date": null,
        "year": 2020,
        "abc_published": null,
        "prop_takers": null,
        "prop_infra": null,
        "time_extension": null
    }
}), db_connection).await,

convert_projects_into_active_model_v2(&json!({
    "id": "508",
    "attributes": {
      "project_status": "Completed",
        "name": "Concreting/Improvement of Barangay Bulatok Road",
        "code": "2SAIP20 - 8000 - C1a5",
        "location": null,
        "cost": 236631.12,
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
        "appropriation": 250000,
        "abc": 0,
        "status_id": {
            "data": null
        },
        "remarks": null,
        "entry_type": "IMPLEMENTED",
        "bid_date": "2020-11-12",
        "year": 2020,
        "abc_published": null,
        "prop_takers": null,
        "prop_infra": null,
        "time_extension": null
    }
}), db_connection).await,

convert_projects_into_active_model_v2(&json!({
    "id": "509",
    "attributes": {
       "project_status": "Completed",
        "name": "Concreting/Improvement of La Suerte Road (Near Barangay Hall Site)",
        "code": "2SAIP20 - 8000 - C1a6",
        "location": null,
        "cost": 677490,
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
        "appropriation": 700000,
        "abc": 0,
        "status_id": {
            "data": null
        },
        "remarks": null,
        "entry_type": "IMPLEMENTED",
        "bid_date": "2020-07-06",
        "year": 2020,
        "abc_published": null,
        "prop_takers": null,
        "prop_infra": null,
        "time_extension": null
    }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
            "id": "510",
            "attributes": {
              "project_status": "Completed",
                "name": "Concreting/Improvement of Lourdes Road (Portion after the Bridge)",
                "code": "2SAIP20 - 8000 - C1a7",
                "location": null,
                "cost": 963613.24,
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
                "appropriation": 1000000,
                "abc": 0,
                "status_id": {
                    "data": null
                },
                "remarks": null,
                "entry_type": "IMPLEMENTED",
                "bid_date": "2020-07-06",
                "year": 2020,
                "abc_published": null,
                "prop_takers": null,
                "prop_infra": null,
                "time_extension": null
            }
        }), db_connection).await, // .await here executes it immediately
        convert_projects_into_active_model_v2(&json!({
            "id": "511",
            "attributes": {
              "project_status": "Not Yet Started",
                "name": "Concreting/Improvement of Road Near Dampalan Spillway (at Difficult Portion)",
                "code": "2SAIP20 - 8000 - C1a8",
                "location": null,
                "cost": 0,
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
                "appropriation": 1000000,
                "abc": 0,
                "status_id": {
                    "data": null
                },
                "remarks": null,
                "entry_type": "IMPLEMENTED",
                "bid_date": "2020-09-28",
                "year": 2020,
                "abc_published": null,
                "prop_takers": null,
                "prop_infra": null,
                "time_extension": null
            }
        }), db_connection).await,
        convert_projects_into_active_model_v2(&json!({
            "id": "512",
            "attributes": {
              "project_status": "Completed",
                "name": "Rehab/Concreting of Road at Lower Sibatang Near Lingue NHS",
                "code": "2SAIP20 - 8000 - C1a9",
                "location": null,
                "cost": 482332.52,
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
                "appropriation": 500000,
                "abc": 0,
                "status_id": {
                    "data": null
                },
                "remarks": null,
                "entry_type": "IMPLEMENTED",
                "bid_date": "2020-08-05",
                "year": 2020,
                "abc_published": null,
                "prop_takers": null,
                "prop_infra": null,
                "time_extension": null
            }
        }), db_connection).await,
        convert_projects_into_active_model_v2(&json!({
            "id": "513",
            "attributes": {
               "project_status": "Not Yet Started",
                "name": "Concreting/Improvement of Access Road to Relocation Center and Housing Site at IBT Site",
                "code": "2SAIP20 - 8000 - C1a10",
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
                "appropriation": 2000000,
                "abc": 0,
                "status_id": {
                    "data": null
                },
                "remarks": null,
                "entry_type": "IMPLEMENTED",
                "bid_date": null,
                "year": 2020,
                "abc_published": null,
                "prop_takers": null,
                "prop_infra": null,
                "time_extension": null
            }
        }), db_connection).await,
        convert_projects_into_active_model_v2(&json!({
            "id": "514",
            "attributes": {
              "project_status": "Completed",
                "name": "Concreting/Improvement of Road (Continuation) Leading to Brgy Muricay (New Route)",
                "code": "2SAIP20 - 8000 - C1a11",
                "location": null,
                "cost": 1062759.78,
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
                "appropriation": 1100000,
                "abc": 0,
                "status_id": {
                    "data": null
                },
                "remarks": null,
                "entry_type": "IMPLEMENTED",
                "bid_date": "2020-08-17",
                "year": 2020,
                "abc_published": null,
                "prop_takers": null,
                "prop_infra": null,
                "time_extension": null
            }
        }), db_connection).await,
        convert_projects_into_active_model_v2(&json!({
            "id": "515",
            "attributes": {
              "project_status": "Completed",
                "name": "Concreting/Improvement of Road with Box Culvert at Lala - Lumad Brgy Road",
                "code": "2SAIP20 - 8000 - C1a12",
                "location": null,
                "cost": 2905853.52,
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
                "appropriation": 3000000,
                "abc": 0,
                "status_id": {
                    "data": null
                },
                "remarks": null,
                "entry_type": "IMPLEMENTED",
                "bid_date": "2020-09-28",
                "year": 2020,
                "abc_published": null,
                "prop_takers": null,
                "prop_infra": null,
                "time_extension": null
            }
        }), db_connection).await,
        convert_projects_into_active_model_v2(&json!({
            "id": "516",
            "attributes": {
              "project_status": "Completed",
                "name": "Construction of Sidewalk with Drainage From St. Nino to Banale Rd",
                "code": "2SAIP20 - 8000 - C1b1",
                "location": null,
                "cost": 3854064.57,
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
                "appropriation": 4000000,
                "abc": 0,
                "status_id": {
                    "data": null
                },
                "remarks": null,
                "entry_type": "IMPLEMENTED",
                "bid_date": "2020-11-23",
                "year": 2020,
                "abc_published": null,
                "prop_takers": null,
                "prop_infra": null,
                "time_extension": null
            }
        }), db_connection).await,
        convert_projects_into_active_model_v2(&json!({
            "id": "517",
            "attributes": {
              "project_status": "Completed",
                "name": "Construction of Sidewalk with Drainage Along RT Lim St (Jct. Urro St to National Highway)",
                "code": "2SAIP20 - 8000 - C1b2",
                "location": null,
                "cost": 1924186.19,
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
                "appropriation": 2000000,
                "abc": 0,
                "status_id": {
                    "data": null
                },
                "remarks": null,
                "entry_type": "IMPLEMENTED",
                "bid_date": "2020-09-21",
                "year": 2020,
                "abc_published": null,
                "prop_takers": null,
                "prop_infra": null,
                "time_extension": null
            }
        }), db_connection).await,
        convert_projects_into_active_model_v2(&json!({
            "id": "518",
            "attributes": {
              "project_status": "Completed",
                "name": "Construction of Sidewalk and Drainage Along Aquino St (Jct Salazar St - Sabate St)",
                "code": "2SAIP20 - 8000 - C1b3",
                "location": "[]",
                "cost": 1998079.94,
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
                "prop_fund": "6",
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
                "bid_date": "2020-09-28",
                "year": 2020,
                "abc_published": null,
                "prop_takers": null,
                "prop_infra": null,
                "time_extension": null
            }
        }), db_connection).await,
        convert_projects_into_active_model_v2(&json!({
            "id": "519",
            "attributes": {
              "project_status": "Completed",
                "name": "Construction of Sidewalk and Drainage Along Sabate St. (Jct FS Pajares Ave -Roxas St)",
                "code": "2SAIP20 - 8000 - C1b4",
                "location": null,
                "cost": 967997.92,
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
                "appropriation": 1000000,
                "abc": 0,
                "status_id": {
                    "data": null
                },
                "remarks": null,
                "entry_type": "IMPLEMENTED",
                "bid_date": "2020-12-29",
                "year": 2020,
                "abc_published": null,
                "prop_takers": null,
                "prop_infra": null,
                "time_extension": null
            }
        }), db_connection).await,
        convert_projects_into_active_model_v2(&json!({
            "id": "520",
            "attributes": {
              "project_status": "Completed",
                "name": "Construction of Sidewalk and Drainage Along Datoc St. (Jct Sagun St - Urro St)",
                "code": "2SAIP20 - 8000 - C1b5",
                "location": null,
                "cost": 2897745.64,
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
                "appropriation": 2000000,
                "abc": 0,
                "status_id": {
                    "data": null
                },
                "remarks": null,
                "entry_type": "IMPLEMENTED",
                "bid_date": "2020-11-23",
                "year": 2020,
                "abc_published": null,
                "prop_takers": null,
                "prop_infra": null,
                "time_extension": null
            }
        }), db_connection).await,
convert_projects_into_active_model_v2(&json!({
            "id": "521",
            "attributes": {
              "project_status": "Bidded",
                "name": "Construction of Sidewalk with Drainage Along National Highway ( Balangasan Bridge - Balintawak Boundary)",
                "code": "2SAIP20 - 8000 - C1b6",
                "location": null,
                "cost": 2902325.45,
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
                "appropriation": 3000000,
                "abc": 0,
                "status_id": {
                    "data": null
                },
                "remarks": null,
                "entry_type": "IMPLEMENTED",
                "bid_date": "2021-06-30",
                "year": 2020,
                "abc_published": null,
                "prop_takers": null,
                "prop_infra": null,
                "time_extension": null
            }
        }), db_connection).await,
convert_projects_into_active_model_v2(&json!({
            "id": "522",
            "attributes": {
              "project_status": "Completed",
                "name": "Construction of Sidewalk with Drainage Along National Highway ( Tiguma - Tawagan Boundary)",
                "code": "2SAIP20 - 8000 - C1b7",
                "location": null,
                "cost": 3845533.99,
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
                "appropriation": 4000000,
                "abc": 0,
                "status_id": {
                    "data": null
                },
                "remarks": null,
                "entry_type": "IMPLEMENTED",
                "bid_date": "2021-01-11",
                "year": 2020,
                "abc_published": null,
                "prop_takers": null,
                "prop_infra": null,
                "time_extension": null
            }
        }), db_connection).await,
convert_projects_into_active_model_v2(&json!({
            "id": "523",
            "attributes": {
              "project_status": "Completed",
                "name": "Construction of Sidewalk with Drainage Along FS Pajares Ave (Sabate - Mercedes St) Fronting Former DILG",
                "code": "2SAIP20 - 8000 - C1b8",
                "location": null,
                "cost": 1453500.17,
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
                "appropriation": 1500000,
                "abc": 0,
                "status_id": {
                    "data": null
                },
                "remarks": null,
                "entry_type": "IMPLEMENTED",
                "bid_date": "2020-09-21",
                "year": 2020,
                "abc_published": null,
                "prop_takers": null,
                "prop_infra": null,
                "time_extension": null
            }
        }), db_connection).await,
convert_projects_into_active_model_v2(&json!({
            "id": "524",
            "attributes": {
              "project_status": "Completed",
                "name": "Construction of Drainage Canal at Bgry Balangasan (Sect Prk Kawayan, Prk Romblon, Prk Riverside and Prk Tabing Ilog)",
                "code": "2SAIP20 - 8000 - C1b9",
                "location": null,
                "cost": 1440244.02,
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
                "appropriation": 2000000,
                "abc": 0,
                "status_id": {
                    "data": null
                },
                "remarks": null,
                "entry_type": "IMPLEMENTED",
                "bid_date": "2021-08-31",
                "year": 2020,
                "abc_published": null,
                "prop_takers": null,
                "prop_infra": null,
                "time_extension": null
            }
        }), db_connection).await,
convert_projects_into_active_model_v2(&json!({
            "id": "525",
            "attributes": {
              "project_status": "Bidded",
                "name": "Reinstallation of Demolished Covered Court to Fishport Compound",
                "code": "2SAIP20 - 8000 - C1c1",
                "location": null,
                "cost": 484049.72,
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
                "appropriation": 500000,
                "abc": 0,
                "status_id": {
                    "data": null
                },
                "remarks": null,
                "entry_type": "IMPLEMENTED",
                "bid_date": "2021-05-28",
                "year": 2020,
                "abc_published": null,
                "prop_takers": null,
                "prop_infra": null,
                "time_extension": null
            }
        }), db_connection).await,
convert_projects_into_active_model_v2(&json!({
            "id": "526",
            "attributes": {
              "project_status": "Completed",
                "name": "Construction of Additional Livestock Area at Fishport Entrance",
                "code": "2SAIP20 - 8000 - C1c2",
                "location": null,
                "cost": 1452960.6,
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
                "appropriation": 1500000,
                "abc": 0,
                "status_id": {
                    "data": null
                },
                "remarks": null,
                "entry_type": "IMPLEMENTED",
                "bid_date": "2020-11-09",
                "year": 2020,
                "abc_published": null,
                "prop_takers": null,
                "prop_infra": null,
                "time_extension": null
            }
        }), db_connection).await,
convert_projects_into_active_model_v2(&json!({
            "id": "527",
            "attributes": {
              "project_status": "Completed",
                "name": "Construction of Perimeter Fence and Rehab of Livestock Building At Fishport",
                "code": "2SAIP20 - 8000 - C1c3",
                "location": null,
                "cost": 489735.96,
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
                "appropriation": 500000,
                "abc": 0,
                "status_id": {
                    "data": null
                },
                "remarks": null,
                "entry_type": "IMPLEMENTED",
                "bid_date": "2021-03-29",
                "year": 2020,
                "abc_published": null,
                "prop_takers": null,
                "prop_infra": null,
                "time_extension": null
            }
        }), db_connection).await,
convert_projects_into_active_model_v2(&json!({
            "id": "528",
            "attributes": {
              "project_status": "Completed",
                "name": "Construction of Concrete Pavement on the Entire Area at City Slaughter House",
                "code": "2SAIP20 - 8000 - C1c4",
                "location": null,
                "cost": 964356.5,
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
                "appropriation": 1000000,
                "abc": 0,
                "status_id": {
                    "data": null
                },
                "remarks": null,
                "entry_type": "IMPLEMENTED",
                "bid_date": "2021-05-28",
                "year": 2020,
                "abc_published": null,
                "prop_takers": null,
                "prop_infra": null,
                "time_extension": null
            }
        }), db_connection).await,
convert_projects_into_active_model_v2(&json!({
            "id": "529",
            "attributes": {
               "project_status": "Completed",
                "name": "Construction of Additional Water Tank at City Slaughter House",
                "code": "2SAIP20 - 8000 - C1c5",
                "location": null,
                "cost": 190999.78,
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
                "appropriation": 200000,
                "abc": 0,
                "status_id": {
                    "data": null
                },
                "remarks": null,
                "entry_type": "IMPLEMENTED",
                "bid_date": "2020-09-08",
                "year": 2020,
                "abc_published": null,
                "prop_takers": null,
                "prop_infra": null,
                "time_extension": null
            }
        }), db_connection).await,
        convert_projects_into_active_model_v2(&json!({
            "id": "530",
            "attributes": {
              "project_status": "Not Yet Started",
                "name": "Conc/Impvt of Boating Area at Boulevard for Tourism Destination",
                "code": "2SAIP20 - 8000 - C1c6",
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
                "appropriation": 2000000,
                "abc": 0,
                "status_id": {
                    "data": null
                },
                "remarks": null,
                "entry_type": "IMPLEMENTED",
                "bid_date": null,
                "year": 2020,
                "abc_published": null,
                "prop_takers": null,
                "prop_infra": null,
                "time_extension": null
            }
        }), db_connection).await,
convert_projects_into_active_model_v2(&json!({
            "id": "531",
            "attributes": {
              "project_status": "Completed",
                "name": "Renovation of Justice Hall (Repainting & Tile Replacement)",
                "code": "2SAIP20 - 8000 - C1c7",
                "location": null,
                "cost": 944599,
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
                "appropriation": 1000000,
                "abc": 0,
                "status_id": {
                    "data": null
                },
                "remarks": null,
                "entry_type": "IMPLEMENTED",
                "bid_date": "2020-11-25",
                "year": 2020,
                "abc_published": null,
                "prop_takers": null,
                "prop_infra": null,
                "time_extension": null
            }
        }), db_connection).await,
convert_projects_into_active_model_v2(&json!({
            "id": "532",
            "attributes": {
              "project_status": "Completed",
                "name": "Const/Rehab of Water System at Barangay Napolan",
                "code": "2SAIP20 - 8000 - C1c8",
                "location": "[]",
                "cost": 248969,
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
                "prop_fund": "6",
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
                "appropriation": 250000,
                "abc": 0,
                "status_id": {
                    "data": null
                },
                "remarks": null,
                "entry_type": "IMPLEMENTED",
                "bid_date": "2020-09-08",
                "year": 2020,
                "abc_published": null,
                "prop_takers": null,
                "prop_infra": null,
                "time_extension": null
            }
        }), db_connection).await,
convert_projects_into_active_model_v2(&json!({
            "id": "533",
            "attributes": {
              "project_status": "Completed",
                "name": "Opening/Concreting of Road Leading to Manga Falls",
                "code": "3SAIP20 - 8000 - B1a",
                "location": null,
                "cost": 5791041.07,
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
                "prop_fund": "7",
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
                "appropriation": 4000000,
                "abc": 0,
                "status_id": {
                    "data": null
                },
                "remarks": null,
                "entry_type": "IMPLEMENTED",
                "bid_date": "2020-09-28",
                "year": 2020,
                "abc_published": null,
                "prop_takers": null,
                "prop_infra": null,
                "time_extension": null
            }
        }), db_connection).await,
convert_projects_into_active_model_v2(&json!({
            "id": "534",
            "attributes": {
              "project_status": "Completed",
                "name": "Construction of Drainage Along Duterte St (Jct Jamisola St - Rizal Ave.) Western side",
                "code": "3SAIP20 - 8000 - B1b",
                "location": null,
                "cost": 978480.42,
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
                "prop_fund": "7",
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
                "bid_date": "2020-12-29",
                "year": 2020,
                "abc_published": null,
                "prop_takers": null,
                "prop_infra": null,
                "time_extension": null
            }
        }), db_connection).await,
convert_projects_into_active_model_v2(&json!({
            "id": "535",
            "attributes": {
               "project_status": "Completed",
                "name": "Concreting of Deborok Barangay Road to Ditoray Barangay Road",
                "code": "3SAIP20 - 8000 - B1c",
                "location": null,
                "cost": 1934683.25,
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
                "prop_fund": "7",
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
                "appropriation": 2000000,
                "abc": 0,
                "status_id": {
                    "data": null
                },
                "remarks": null,
                "entry_type": "IMPLEMENTED",
                "bid_date": "2020-08-17",
                "year": 2020,
                "abc_published": null,
                "prop_takers": null,
                "prop_infra": null,
                "time_extension": null
            }
        }), db_connection).await,
convert_projects_into_active_model_v2(&json!({
            "id": "536",
            "attributes": {
              "project_status": "Completed",
                "name": "Concreting of Road from Barangay Tiguma Road Going to Barangay White Beach ( Continuation)",
                "code": "3SAIP20 - 8000 - B1d",
                "location": null,
                "cost": 2903231.8,
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
                "prop_fund": "7",
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
                "appropriation": 3000000,
                "abc": 0,
                "status_id": {
                    "data": null
                },
                "remarks": null,
                "entry_type": "IMPLEMENTED",
                "bid_date": "2020-09-28",
                "year": 2020,
                "abc_published": null,
                "prop_takers": null,
                "prop_infra": null,
                "time_extension": null
            }
        }), db_connection).await,
convert_projects_into_active_model_v2(&json!({
            "id": "537",
            "attributes": {
              "project_status": "Not Yet Started",
                "name": "Development of Manga Falls",
                "code": "3SAIP20 - 8000 - B2a",
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
                "prop_fund": "7",
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
                "year": 2020,
                "abc_published": null,
                "prop_takers": null,
                "prop_infra": null,
                "time_extension": null
            }
        }), db_connection).await,
convert_projects_into_active_model_v2(&json!({
            "id": "538",
            "attributes": {
              "project_status": "On-Going",
                "name": "Development of Water Park at Boulevadr, Brgy Santiago",
                "code": "3SAIP20-8000-B2b",
                "location": "[\"16\"]",
                "cost": 0,
                "contractor": {
                    "data": null
                },
                "duration": null,
                "adjusted": null,
                "total": null,
                "weight": null,
                "prop_sdg": "[]",
                "prop_fund": "7",
                "prop_type": "11",
                "prop_category": "1",
                "prop_sector": "[]",
                "prop_assign": "2",
                "accom_total": 20,
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
                "remarks": "20% accomplishment as of Aug. 2, 2024",
                "entry_type": "IMPLEMENTED",
                "bid_date": null,
                "year": 2020,
                "abc_published": null,
                "prop_takers": null,
                "prop_infra": null,
                "time_extension": null
            }
        }), db_connection).await,
convert_projects_into_active_model_v2(&json!({
            "id": "539",
            "attributes": {
              "project_status": "Not Yet Started",
                "name": "Widening of Lourdes Barangay Road to Lison Valley",
                "code": "4SAIP20 - 8000R - B1a1",
                "location": null,
                "cost": 2323442.68,
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
                "prop_fund": "8",
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
                "appropriation": 2500000,
                "abc": 0,
                "status_id": {
                    "data": null
                },
                "remarks": null,
                "entry_type": "IMPLEMENTED",
                "bid_date": "2020-11-09",
                "year": 2020,
                "abc_published": null,
                "prop_takers": null,
                "prop_infra": null,
                "time_extension": null
            }
        }), db_connection).await,
        convert_projects_into_active_model_v2(&json!({
  "id": "540",
  "attributes": {
    "project_status": "Completed",
    "name": "Road Opening/Concreting of Datagan Barangay Road - Kendis Cave to Quarry Site Road",
    "code": "4SAIP20 - 8000R - B1a2",
    "location": null,
    "cost": 4356812.5,
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
    "prop_fund": "8",
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
    "appropriation": 4500000,
    "abc": 0,
    "status_id": {
      "data": null
    },
    "remarks": null,
    "entry_type": "IMPLEMENTED",
    "bid_date": "2021-03-15",
    "year": 2020,
    "abc_published": null,
    "prop_takers": null,
    "prop_infra": null,
    "time_extension": null
  }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
  "id": "541",
  "attributes": {
     "project_status": "Completed",
    "name": "Construction of Integrated Bus Terminal (IBT) Annex Phase II, Sta. Lucia",
    "code": "4SAIP20 - 8000R - B1b1",
    "location": null,
    "cost": 1440942.1,
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
    "prop_sdg": null,
    "prop_fund": "8",
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
    "appropriation": 1500000,
    "abc": 0,
    "status_id": {
      "data": null
    },
    "remarks": null,
    "entry_type": "IMPLEMENTED",
    "bid_date": "2020-09-21",
    "year": 2020,
    "abc_published": null,
    "prop_takers": null,
    "prop_infra": null,
    "time_extension": null
  }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
  "id": "542",
  "attributes": {
    "project_status": "Completed",
    "name": "Construction of Integrated Bus Terminal (IBT) Annex Phase III, Sta. Lucia",
    "code": "4SAIP20 - 8000R - B1b2",
    "location": null,
    "cost": 3486716.97,
    "contractor": {
      "data": {
        "id": "37",
        "attributes": {
          "name": "ILLANA Bay Construction",
          "street_purok": null
        }
      }
    },
    "duration": null,
    "adjusted": null,
    "total": null,
    "weight": null,
    "prop_sdg": null,
    "prop_fund": "8",
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
    "appropriation": 3600000,
    "abc": 0,
    "status_id": {
      "data": null
    },
    "remarks": null,
    "entry_type": "IMPLEMENTED",
    "bid_date": "2020-12-09",
    "year": 2020,
    "abc_published": null,
    "prop_takers": null,
    "prop_infra": null,
    "time_extension": null
  }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
  "id": "543",
  "attributes": {
    "project_status": "Completed",
    "name": "Installation of Electrical Connection at IBT Annex, Brgy Sta. Lucia",
    "code": "4SAIP20 - 8000R - B1b3",
    "location": null,
    "cost": 484009.75,
    "contractor": {
      "data": null
    },
    "duration": null,
    "adjusted": null,
    "total": null,
    "weight": null,
    "prop_sdg": null,
    "prop_fund": "8",
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
    "bid_date": "2021-09-07",
    "year": 2020,
    "abc_published": null,
    "prop_takers": null,
    "prop_infra": null,
    "time_extension": null
  }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
  "id": "544",
  "attributes": {
    "project_status": "Completed",
    "name": "Landscaping of Plaza Luz & City Hall Grounds",
    "code": "4SAIP20 - 8000R - B1c1",
    "location": null,
    "cost": 1975017.62,
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
    "prop_fund": "8",
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
    "appropriation": 2059140,
    "abc": 0,
    "status_id": {
      "data": null
    },
    "remarks": null,
    "entry_type": "IMPLEMENTED",
    "bid_date": "2021-03-10",
    "year": 2020,
    "abc_published": null,
    "prop_takers": null,
    "prop_infra": null,
    "time_extension": null
  }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
  "id": "545",
  "attributes": {
    "project_status": "Completed",
    "name": "Construction of Perimeter Fence and Gates at Plaza Luz",
    "code": "4SAIP20 - 8000R - B1c2",
    "location": null,
    "cost": 1857656.32,
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
    "prop_fund": "8",
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
    "appropriation": 2000000,
    "abc": 0,
    "status_id": {
      "data": null
    },
    "remarks": null,
    "entry_type": "IMPLEMENTED",
    "bid_date": "2021-06-18",
    "year": 2020,
    "abc_published": null,
    "prop_takers": null,
    "prop_infra": null,
    "time_extension": null
  }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
  "id": "546",
  "attributes": {
    "project_status": "Completed",
    "name": "Concreting of Jamisola St. Jct. Aquino St to Duterte St",
    "code": "5SAIP20 - 8000R - A1a",
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
    "prop_fund": "12",
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
    "remarks": "100% physically completed",
    "entry_type": "IMPLEMENTED",
    "bid_date": null,
    "year": 2020,
    "abc_published": null,
    "prop_takers": null,
    "prop_infra": null,
    "time_extension": null
  }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
  "id": "547",
  "attributes": {
    "project_status": "Completed",
    "name": "Concreting of Rotunda Parking Lot",
    "code": "5SAIP20 - 8000R - A1b",
    "location": null,
    "cost": 1913790,
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
    "prop_fund": "12",
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
    "appropriation": 2000000,
    "abc": 0,
    "status_id": {
      "data": null
    },
    "remarks": null,
    "entry_type": "IMPLEMENTED",
    "bid_date": "2021-04-05",
    "year": 2020,
    "abc_published": null,
    "prop_takers": null,
    "prop_infra": null,
    "time_extension": null
  }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
  "id": "548",
  "attributes": {
    "project_status": "Completed",
    "name": "Construction of Rotunda Kitchen",
    "code": "5SAIP20-8000R-A1c/joint fund w/3SAIP21-8000R-C-1c10",
    "location": "[]",
    "cost": 4156034.58,
    "contractor": {
      "data": {
        "id": "37",
        "attributes": {
          "name": "ILLANA Bay Construction",
          "street_purok": null
        }
      }
    },
    "duration": 180,
    "adjusted": null,
    "total": null,
    "weight": null,
    "prop_sdg": "[]",
    "prop_fund": "12",
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
    "start": "2022-01-31",
    "target": "2022-07-30",
    "office": "Engineer's Office",
    "assigned": null,
    "appropriation": 1500000,
    "abc": 0,
    "status_id": {
      "data": null
    },
    "remarks": "100% physically completed based on approved technical plans & specifications\n",
    "entry_type": "IMPLEMENTED",
    "bid_date": "2021-11-08",
    "year": 2020,
    "abc_published": null,
    "prop_takers": null,
    "prop_infra": null,
    "time_extension": null
  }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
  "id": "549",
  "attributes": {
    "project_status": "Completed",
    "name": "Construction of Rotunda Stage",
    "code": "5SAIP20 - 8000R - A1d",
    "location": null,
    "cost": 1967367.77,
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
    "prop_fund": "12",
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
    "appropriation": 2000000,
    "abc": 0,
    "status_id": {
      "data": null
    },
    "remarks": null,
    "entry_type": "IMPLEMENTED",
    "bid_date": "2021-01-19",
    "year": 2020,
    "abc_published": null,
    "prop_takers": null,
    "prop_infra": null,
    "time_extension": null
  }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
  "id": "550",
  "attributes": {
    "project_status": "Completed",
    "name": "Improvement of Plaza Luz",
    "code": "5SAIP20 - 8000R - A1e",
    "location": null,
    "cost": 1426822.69,
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
    "prop_fund": "12",
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
    "appropriation": 1500000,
    "abc": 0,
    "status_id": {
      "data": null
    },
    "remarks": null,
    "entry_type": "IMPLEMENTED",
    "bid_date": "2021-03-10",
    "year": 2020,
    "abc_published": null,
    "prop_takers": null,
    "prop_infra": null,
    "time_extension": null
  }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
  "id": "551",
  "attributes": {
    "project_status": "Completed",
    "name": "Concreting of Motorpool Flooring",
    "code": "5SAIP20 - 8000R - A1f",
    "location": null,
    "cost": 968982.65,
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
    "prop_fund": "12",
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
    "bid_date": "2020-12-08",
    "year": 2020,
    "abc_published": null,
    "prop_takers": null,
    "prop_infra": null,
    "time_extension": null
  }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
  "id": "552",
  "attributes": {
    "project_status": "Not Yet Started",
    "name": "Renovation/Improvement of San Francisco Brgy Hall",
    "code": "6SAIP20 - 8000R - A1",
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
    "prop_fund": "13",
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
    "appropriation": 700000,
    "abc": 0,
    "status_id": {
      "data": null
    },
    "remarks": null,
    "entry_type": "IMPLEMENTED",
    "bid_date": null,
    "year": 2020,
    "abc_published": null,
    "prop_takers": null,
    "prop_infra": null,
    "time_extension": null
  }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
  "id": "553",
  "attributes": {
    "project_status": "Completed",
    "name": "Construction of SP Building Phase II",
    "code": "6SAIP20 - 8000R - A2",
    "location": null,
    "cost": 9580509.26,
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
    "prop_sdg": null,
    "prop_fund": "13",
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
    "bid_date": "2022-02-16",
    "year": 2020,
    "abc_published": null,
    "prop_takers": null,
    "prop_infra": null,
    "time_extension": null
  }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
  "id": "554",
  "attributes": {
    "project_status": "Completed",
    "name": "Concreting/Improvement of Kagawasan - Alegria Barangay Road",
    "code": "7SAIP20 - 8000R - B1 - a",
    "location": null,
    "cost": 1492366.86,
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
    "prop_fund": "14",
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
    "appropriation": 1500000,
    "abc": 0,
    "status_id": {
      "data": null
    },
    "remarks": null,
    "entry_type": "IMPLEMENTED",
    "bid_date": null,
    "year": 2020,
    "abc_published": null,
    "prop_takers": null,
    "prop_infra": null,
    "time_extension": null
  }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
  "id": "555",
  "attributes": {
    "project_status": "Completed",
    "name": "Finishing Works at Darussalam Wall",
    "code": "7SAIP20 - 8000R - B1 - b",
    "location": null,
    "cost": 1583900.16,
    "contractor": {
      "data": {
        "id": "37",
        "attributes": {
          "name": "ILLANA Bay Construction",
          "street_purok": null
        }
      }
    },
    "duration": null,
    "adjusted": null,
    "total": null,
    "weight": null,
    "prop_sdg": null,
    "prop_fund": "14",
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
    "appropriation": 1650000,
    "abc": 0,
    "status_id": {
      "data": null
    },
    "remarks": null,
    "entry_type": "IMPLEMENTED",
    "bid_date": "2021-01-11",
    "year": 2020,
    "abc_published": null,
    "prop_takers": null,
    "prop_infra": null,
    "time_extension": null
  }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
  "id": "556",
  "attributes": {
     "project_status": "Completed",
    "name": "Widening of Kagawasan - Danlugan to proper (Continuation)",
    "code": "7SAIP20 - 8000R - B1 - c",
    "location": null,
    "cost": 1936049.04,
    "contractor": {
      "data": {
        "id": "36",
        "attributes": {
          "name": "CDTech Builders",
          "street_purok": null
        }
      }
    },
    "duration": null,
    "adjusted": null,
    "total": null,
    "weight": null,
    "prop_sdg": null,
    "prop_fund": "14",
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
    "appropriation": 2000000,
    "abc": 0,
    "status_id": {
      "data": null
    },
    "remarks": null,
    "entry_type": "IMPLEMENTED",
    "bid_date": "2021-04-05",
    "year": 2020,
    "abc_published": null,
    "prop_takers": null,
    "prop_infra": null,
    "time_extension": null
  }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
  "id": "557",
  "attributes": {
    "project_status": "Completed",
    "name": "Improvement of Covered Court At City Plaza",
    "code": "7SAIP20 - 8000R - B1 - d",
    "location": null,
    "cost": 966299.95,
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
    "prop_fund": "14",
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
    "bid_date": "2020-12-29",
    "year": 2020,
    "abc_published": null,
    "prop_takers": null,
    "prop_infra": null,
    "time_extension": null
  }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
  "id": "558",
  "attributes": {
    "project_status": "Completed",
    "name": "Concreting of Road at Kadena de Amor Leading to Kagawasan Road",
    "code": "2SAIL20-R8000-1A1",
    "location": null,
    "cost": 479921.6,
    "contractor": {
      "data": {
        "id": "45",
        "attributes": {
          "name": "Pyramid Builders",
          "street_purok": null
        }
      }
    },
    "duration": null,
    "adjusted": null,
    "total": null,
    "weight": null,
    "prop_sdg": null,
    "prop_fund": "17",
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
    "bid_date": "2020-11-05",
    "year": 2020,
    "abc_published": null,
    "prop_takers": null,
    "prop_infra": null,
    "time_extension": null
  }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
  "id": "559",
  "attributes": {
    "project_status": "Completed",
    "name": "Concreting of Road at Zone 2 Malipayon From Brgy Hall to North Diversion Road",
    "code": "2SAIL20-R8000-1A2",
    "location": null,
    "cost": 1255694.46,
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
    "prop_sdg": null,
    "prop_fund": "17",
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
    "bid_date": "2020-11-09",
    "year": 2020,
    "abc_published": null,
    "prop_takers": null,
    "prop_infra": null,
    "time_extension": null
  }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
  "id": "560",
  "attributes": {
    "project_status": "Completed",
    "name": "Rehab of Road From Purok Rosas to Prk Bombil, Brgy. Bulawan",
    "code": "2SAIL20-R8000-1A3",
    "location": null,
    "cost": 871313.55,
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
    "prop_fund": "17",
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
    "appropriation": 900000,
    "abc": 0,
    "status_id": {
      "data": null
    },
    "remarks": null,
    "entry_type": "IMPLEMENTED",
    "bid_date": "2020-11-25",
    "year": 2020,
    "abc_published": null,
    "prop_takers": null,
    "prop_infra": null,
    "time_extension": null
  }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
  "id": "561",
  "attributes": {
     "project_status": "Completed",
    "name": "Widening of Road From Kagawasan to Danlugan Brgy Road",
    "code": "2SAIL20-R8000-1A4",
    "location": null,
    "cost": 1446810.66,
    "contractor": {
      "data": {
        "id": "36",
        "attributes": {
          "name": "CDTech Builders",
          "street_purok": null
        }
      }
    },
    "duration": null,
    "adjusted": null,
    "total": null,
    "weight": null,
    "prop_sdg": null,
    "prop_fund": "17",
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
    "appropriation": 1500000,
    "abc": 0,
    "status_id": {
      "data": null
    },
    "remarks": null,
    "entry_type": "IMPLEMENTED",
    "bid_date": "2020-11-09",
    "year": 2020,
    "abc_published": null,
    "prop_takers": null,
    "prop_infra": null,
    "time_extension": null
  }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
  "id": "562",
  "attributes": {
    "project_status": "Completed",
    "name": "Concreting/Improvement of Road at Prk Subida to Brgy Banale",
    "code": "2SAIL20-R8000-1A5",
    "location": null,
    "cost": 967378.05,
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
    "prop_fund": "17",
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
    "bid_date": "2020-11-12",
    "year": 2020,
    "abc_published": null,
    "prop_takers": null,
    "prop_infra": null,
    "time_extension": null
  }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
  "id": "563",
  "attributes": {
     "project_status": "Not Yet Started",
    "name": "Construction of Alley at Brgy Dumagoc",
    "code": "2SAIL20-R8000-1A6",
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
    "prop_fund": "17",
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
    "year": 2020,
    "abc_published": null,
    "prop_takers": null,
    "prop_infra": null,
    "time_extension": null
  }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
  "id": "564",
  "attributes": {
    "project_status": "Completed",
    "name": "Road Concreting Leading to E/S Road Gubac",
    "code": "2SAIL20-R8000-1A7",
    "location": null,
    "cost": 482987,
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
    "prop_fund": "17",
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
    "bid_date": "2020-10-27",
    "year": 2020,
    "abc_published": null,
    "prop_takers": null,
    "prop_infra": null,
    "time_extension": null
  }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
  "id": "565",
  "attributes": {
    "project_status": "Completed",
    "name": "Conc/Impvt of Road From Brgy Hall to Lumatao at Brgy. Gubang",
    "code": "2SAIL20-R8000-1A8",
    "location": "[]",
    "cost": 974399.49,
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
    "prop_fund": "17",
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
    "appropriation": 1000000,
    "abc": 0,
    "status_id": {
      "data": null
    },
    "remarks": null,
    "entry_type": "IMPLEMENTED",
    "bid_date": "2020-11-04",
    "year": 2020,
    "abc_published": null,
    "prop_takers": null,
    "prop_infra": null,
    "time_extension": null
  }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
  "id": "566",
  "attributes": {
    "project_status": "Completed",
    "name": "Conc/Impvt of Road at Lower Kahayagan",
    "code": "2SAIL20-R8000-1A9",
    "location": null,
    "cost": 961612.87,
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
    "prop_sdg": null,
    "prop_fund": "17",
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
    "bid_date": "2020-11-04",
    "year": 2020,
    "abc_published": null,
    "prop_takers": null,
    "prop_infra": null,
    "time_extension": null
  }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
  "id": "567",
  "attributes": {
     "project_status": "Completed",
    "name": "Conc/Impvt of Road Beside ZSSSAT School, Kawit",
    "code": "2SAIL20-R8000-1A10",
    "location": null,
    "cost": 0,
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
    "prop_fund": "17",
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
    "bid_date": "2020-12-08",
    "year": 2020,
    "abc_published": null,
    "prop_takers": null,
    "prop_infra": null,
    "time_extension": null
  }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
  "id": "568",
  "attributes": {
    "project_status": "Completed",
    "name": "Conc/Impvt of Road at prk Sandayong. Brgy. Lapedian",
    "code": "2SAIL20-R8000-1A11",
    "location": null,
    "cost": 677750.7,
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
    "prop_fund": "17",
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
    "appropriation": 700000,
    "abc": 0,
    "status_id": {
      "data": null
    },
    "remarks": null,
    "entry_type": "IMPLEMENTED",
    "bid_date": "2020-11-12",
    "year": 2020,
    "abc_published": null,
    "prop_takers": null,
    "prop_infra": null,
    "time_extension": null
  }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
  "id": "569",
  "attributes": {
    "project_status": "Completed",
    "name": "Gravelling of Road at prk Santol to Woodland La Suerte",
    "code": "2SAIL20-R8000-1A12",
    "location": null,
    "cost": 289968,
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
    "prop_fund": "17",
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
    "bid_date": "2020-11-12",
    "year": 2020,
    "abc_published": null,
    "prop_takers": null,
    "prop_infra": null,
    "time_extension": null
  }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
  "id": "570",
  "attributes": {
    "project_status": "Completed",
    "name": "Conc. Of Road at Prk Nagka to Brgy Bulatok, Brgy. Lenienza",
    "code": "2SAIL20-R8000-1A13",
    "location": null,
    "cost": 480149.72,
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
    "prop_sdg": null,
    "prop_fund": "17",
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
    "bid_date": "2020-10-27",
    "year": 2020,
    "abc_published": null,
    "prop_takers": null,
    "prop_infra": null,
    "time_extension": null
  }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
  "id": "571",
  "attributes": {
    "project_status": "Completed",
    "name": "Conc of Road at Purok Mangga, Lenienza",
    "code": "2SAIL20-R8000-1A14",
    "location": null,
    "cost": 482993.11,
    "contractor": {
      "data": {
        "id": "46",
        "attributes": {
          "name": "Danton Construction",
          "street_purok": null
        }
      }
    },
    "duration": null,
    "adjusted": null,
    "total": null,
    "weight": null,
    "prop_sdg": null,
    "prop_fund": "17",
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
    "bid_date": "2020-11-05",
    "year": 2020,
    "abc_published": null,
    "prop_takers": null,
    "prop_infra": null,
    "time_extension": null
  }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
  "id": "572",
  "attributes": {
    "project_status": "Completed",
    "name": "Opening of Road from Waling-Waling to Brgy Pedolunan",
    "code": "2SAIL20-R8000-1A15",
    "location": null,
    "cost": 483999,
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
    "prop_fund": "17",
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
    "bid_date": "2021-09-21",
    "year": 2020,
    "abc_published": null,
    "prop_takers": null,
    "prop_infra": null,
    "time_extension": null
  }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
  "id": "573",
  "attributes": {
    "project_status": "Completed",
    "name": "Gravelling/Impvt of Road from Prk talong, brgy Lumad",
    "code": "2SAIL20-R8000-1A16",
    "location": null,
    "cost": 478992,
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
    "prop_fund": "17",
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
    "bid_date": "2021-04-20",
    "year": 2020,
    "abc_published": null,
    "prop_takers": null,
    "prop_infra": null,
    "time_extension": null
  }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
  "id": "574",
  "attributes": {
    "project_status": "Completed",
    "name": "GRAVELLING OF ROAD AT PUROK KAMONGGAY, BRGY LUMAD (FORMERLY CONC/IMPVT OF ROAD AT PRK KAMATIS, BRGY LUMAD)",
    "code": "2SAIL20-R8000-1A17",
    "location": "[\"43\"]",
    "cost": 477499.4,
    "contractor": {
      "data": {
        "id": "10",
        "attributes": {
          "name": "RSQ Construction",
          "street_purok": null
        }
      }
    },
    "duration": 45,
    "adjusted": null,
    "total": null,
    "weight": null,
    "prop_sdg": "[\"13\"]",
    "prop_fund": "24",
    "prop_type": "33",
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
    "start": "2021-08-12",
    "target": "2021-09-26",
    "office": "Engineer's Office",
    "assigned": null,
    "appropriation": 500000,
    "abc": 0,
    "status_id": {
      "data": null
    },
    "remarks": "\"* 100%  physically completed based on approved technical plans & specifications\n* Adjusted target date of Completion: 4/26/2022\"\n* ACTUAL Completion Date: 4/26/2022",
    "entry_type": "IMPLEMENTED",
    "bid_date": "2021-04-20",
    "year": 2020,
    "abc_published": null,
    "prop_takers": null,
    "prop_infra": null,
    "time_extension": null
  }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
  "id": "575",
  "attributes": {
    "project_status": "Completed",
    "name": "Conc/Impvt of Road Fisherville/Hiniusa/Prk Alindahaw, Lumbia",
    "code": "2SAIL20-R8000-1A18",
    "location": null,
    "cost": 1157408.11,
    "contractor": {
      "data": {
        "id": "47",
        "attributes": {
          "name": "Diaca Construction",
          "street_purok": null
        }
      }
    },
    "duration": null,
    "adjusted": null,
    "total": null,
    "weight": null,
    "prop_sdg": null,
    "prop_fund": "17",
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
    "appropriation": 1200000,
    "abc": 0,
    "status_id": {
      "data": null
    },
    "remarks": null,
    "entry_type": "IMPLEMENTED",
    "bid_date": "2020-11-23",
    "year": 2020,
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