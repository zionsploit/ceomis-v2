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
  "id": "576",
  "attributes": {
     "project_status": "Completed",
    "name": "Conc/Impvt of Road from Junction Leading to E/S Brgy Macasing",
    "code": "2SAIL20-R8000-1A19",
    "location": null,
    "cost": 0,
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
    "appropriation": 800000,
    "abc": 0,
    "status_id": {
      "data": null
    },
    "remarks": null,
    "entry_type": "IMPLEMENTED",
    "bid_date": "2021-01-29",
    "year": 2020,
    "abc_published": null,
    "prop_takers": null,
    "prop_infra": null,
    "time_extension": null
  }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
  "id": "577",
  "attributes": {
    "project_status": "Completed",
    "name": "Conc/Impvt of Road at prk Donggoan, Prk Baybay, Napolan",
    "code": "2SAIL20-R8000-1A20",
    "location": null,
    "cost": 770000.01,
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
    "appropriation": 800000,
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
  "id": "578",
  "attributes": {
    "project_status": "Completed",
    "name": "Gravelling of Road at Prk Nangka, Pedolunan",
    "code": "2SAIL20-R8000-1A21",
    "location": null,
    "cost": 241226,
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
    "appropriation": 250000,
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
  "id": "579",
  "attributes": {
    "project_status": "Completed",
    "name": "Conc/Impvt of Road at Lacturan (Westward) brgy. San Pedro",
    "code": "2SAIL20-R8000-1A22",
    "location": "[\"13\"]",
    "cost": 965985.43,
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
    "prop_fund": "25",
    "prop_type": "1",
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
    "start": "2021-01-25",
    "target": "2021-03-26",
    "office": "Engineer's Office",
    "assigned": null,
    "appropriation": 1000000,
    "abc": 0,
    "status_id": {
      "data": null
    },
    "remarks": "100%  physically completed based on approved technical plans & specifications\n",
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
  "id": "580",
  "attributes": {
    "project_status": "Completed",
    "name": "Conc/Impvt of Road at Bay View Heights to Prk Rubia",
    "code": "2SAIL20-R8000-1A23",
    "location": null,
    "cost": 482491.7,
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
  "id": "581",
  "attributes": {
    "project_status": "Completed",
    "name": "Conc/Impvt of Road at Prk Orchid Beside ZSNHS, Paglaum Road",
    "code": "2SAIL20-R8000-1A24",
    "location": "[]",
    "cost": 487013.3,
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
  "id": "582",
  "attributes": {
    "project_status": "Completed",
    "name": "Conc/Impvt of Barangay Alley at Prk Cacao, Tuburan",
    "code": "2SAIL20-R8000-1A25",
    "location": null,
    "cost": 1450467,
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
    "bid_date": "2021-01-11",
    "year": 2020,
    "abc_published": null,
    "prop_takers": null,
    "prop_infra": null,
    "time_extension": null
  }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
  "id": "583",
  "attributes": {
    "project_status": "Not Yet Started",
    "name": "Conc/Impvt of Road to Solar Dryer, Brgy Tulangan",
    "code": "2SAIL20-R8000-1A26",
    "location": null,
    "cost": 381599.88,
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
    "appropriation": 400000,
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
  "id": "584",
  "attributes": {
     "project_status": "Completed",
    "name": "Rehabilitation of Road at Brgy. Tulawas",
    "code": "2SAIL20-R8000-1A27",
    "location": null,
    "cost": 96850.1,
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
    "bid_date": "2020-12-08",
    "year": 2020,
    "abc_published": null,
    "prop_takers": null,
    "prop_infra": null,
    "time_extension": null
  }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
  "id": "585",
  "attributes": {
    "project_status": "Completed",
    "name": "Conc/Impvt of Road Leading to Brgy Cemetery, brgy Tulawas",
    "code": "2SAIL20-R8000-1A28",
    "location": null,
    "cost": 481791,
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
    "appropriation": 500000,
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
  "id": "586",
  "attributes": {
    "project_status": "Completed",
    "name": "Rehabilitation of Road Approaching Tulawas Bridge",
    "code": "2SAIL20-R8000-1A29",
    "location": null,
    "cost": 95116.88,
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
    "bid_date": "2020-11-25",
    "year": 2020,
    "abc_published": null,
    "prop_takers": null,
    "prop_infra": null,
    "time_extension": null
  }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
  "id": "587",
  "attributes": {
    "project_status": "Completed",
    "name": "Widening of Road (1m Both Sides at national Highway Entrance to Upper Sibatang",
    "code": "2SAIL20-R8000-1A30",
    "location": null,
    "cost": 1158928.8,
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
    "appropriation": 1200000,
    "abc": 0,
    "status_id": {
      "data": null
    },
    "remarks": null,
    "entry_type": "IMPLEMENTED",
    "bid_date": "2021-04-26",
    "year": 2020,
    "abc_published": null,
    "prop_takers": null,
    "prop_infra": null,
    "time_extension": null
  }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
  "id": "588",
  "attributes": {
    "project_status": "Completed",
    "name": "Conc/Impvt of Main Alley Connecting to Mini Port, White Beach",
    "code": "2SAIL20-R8000-1A31",
    "location": null,
    "cost": 709134.23,
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
    "appropriation": 710000,
    "abc": 0,
    "status_id": {
      "data": null
    },
    "remarks": null,
    "entry_type": "IMPLEMENTED",
    "bid_date": "2021-01-21",
    "year": 2020,
    "abc_published": null,
    "prop_takers": null,
    "prop_infra": null,
    "time_extension": null
  }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
  "id": "589",
  "attributes": {
    "project_status": "Completed",
    "name": "Improvement of Drainage & Sidewalk Fronting Danlugan Brgy Hall",
    "code": "2SAIL20-R8000-1B1",
    "location": null,
    "cost": 2387417.75,
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
    "appropriation": 2500000,
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
  "id": "590",
  "attributes": {
    "project_status": "Completed",
    "name": "RCCP Installation at Brgy. Ditoray",
    "code": "2SAIL20-R8000-1B2",
    "location": null,
    "cost": 94715,
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
    "appropriation": 100000,
    "abc": 0,
    "status_id": {
      "data": null
    },
    "remarks": null,
    "entry_type": "IMPLEMENTED",
    "bid_date": "2021-04-28",
    "year": 2020,
    "abc_published": null,
    "prop_takers": null,
    "prop_infra": null,
    "time_extension": null
  }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
  "id": "591",
  "attributes": {
    "project_status": "Completed",
    "name": "Repair of Prk Kapalaran Footbridge",
    "code": "2SAIL20-R8000-1B3",
    "location": null,
    "cost": 286990.95,
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
    "appropriation": 300000,
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
  "id": "592",
  "attributes": {
    "project_status": "Completed",
    "name": "Const of Open Canal and Drainage at Brgy Dumagoc",
    "code": "2SAIL20-R8000-1B4",
    "location": null,
    "cost": 480648.89,
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
    "bid_date": "2021-05-28",
    "year": 2020,
    "abc_published": null,
    "prop_takers": null,
    "prop_infra": null,
    "time_extension": null
  }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
  "id": "593",
  "attributes": {
    "project_status": "Completed",
    "name": "Const of Drainage at Prk Papaya brgy Lumbia",
    "code": "2SAIL20-R8000-1B5",
    "location": null,
    "cost": 192433.5,
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
    "appropriation": 200000,
    "abc": 0,
    "status_id": {
      "data": null
    },
    "remarks": null,
    "entry_type": "IMPLEMENTED",
    "bid_date": "2021-03-23",
    "year": 2020,
    "abc_published": null,
    "prop_takers": null,
    "prop_infra": null,
    "time_extension": null
  }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
  "id": "594",
  "attributes": {
    "project_status": "Completed",
    "name": "Const of Multi-Purpose Hall at Brgy Baloyboan",
    "code": "2SAIL20-R8000-1C1",
    "location": "[\"21\"]",
    "cost": 764998.16,
    "contractor": {
      "data": {
        "id": "1",
        "attributes": {
          "name": "CTG Construction",
          "street_purok": "Purok 1"
        }
      }
    },
    "duration": 50,
    "adjusted": null,
    "total": null,
    "weight": null,
    "prop_sdg": "[]",
    "prop_fund": "17",
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
    "start": "2021-05-30",
    "target": "2021-07-19",
    "office": "Engineer's Office",
    "assigned": null,
    "appropriation": 800000,
    "abc": 0,
    "status_id": {
      "data": null
    },
    "remarks": "100% physically completed based on approved technical plans & specifications\n",
    "entry_type": "IMPLEMENTED",
    "bid_date": "2021-04-30",
    "year": 2020,
    "abc_published": null,
    "prop_takers": null,
    "prop_infra": null,
    "time_extension": null
  }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
  "id": "595",
  "attributes": {
     "project_status": "Completed",
    "name": "Const of Brgy Stage at Brgy Hall, Bogo",
    "code": "2SAIL20-R8000-1C2",
    "location": null,
    "cost": 238925.26,
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
    "bid_date": "2021-03-03",
    "year": 2020,
    "abc_published": null,
    "prop_takers": null,
    "prop_infra": null,
    "time_extension": null
  }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
  "id": "596",
  "attributes": {
    "project_status": "Completed",
    "name": "Construction of Multi-Purpose Hall at Brgy. Hall, Bomba",
    "code": "2SAIL20-R8000-1C3",
    "location": null,
    "cost": 775548.16,
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
    "appropriation": 800000,
    "abc": 0,
    "status_id": {
      "data": null
    },
    "remarks": null,
    "entry_type": "IMPLEMENTED",
    "bid_date": "2021-03-30",
    "year": 2020,
    "abc_published": null,
    "prop_takers": null,
    "prop_infra": null,
    "time_extension": null
  }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
  "id": "597",
  "attributes": {
    "project_status": "Completed",
    "name": "Construction of Senior Citizen Office at Brgy Ditoray",
    "code": "2SAIL20-R8000-1C4",
    "location": null,
    "cost": 191052.89,
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
    "appropriation": 200000,
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
  "id": "598",
  "attributes": {
    "project_status": "Completed",
    "name": "Const of Stage and Bleacher at Brgy Ditoray",
    "code": "2SAIL20-R8000-1C5",
    "location": null,
    "cost": 670849.7,
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
    "appropriation": 700000,
    "abc": 0,
    "status_id": {
      "data": null
    },
    "remarks": null,
    "entry_type": "IMPLEMENTED",
    "bid_date": "2021-04-28",
    "year": 2020,
    "abc_published": null,
    "prop_takers": null,
    "prop_infra": null,
    "time_extension": null
  }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
  "id": "599",
  "attributes": {
    "project_status": "Completed",
    "name": "Const of Freedom Stage with CR at Covered Court, Brgy Dumagoc",
    "code": "2SAIL20-R8000-1C6",
    "location": null,
    "cost": 477233.85,
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
    "appropriation": 500000,
    "abc": 0,
    "status_id": {
      "data": null
    },
    "remarks": null,
    "entry_type": "IMPLEMENTED",
    "bid_date": "2021-01-21",
    "year": 2020,
    "abc_published": null,
    "prop_takers": null,
    "prop_infra": null,
    "time_extension": null
  }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
  "id": "600",
  "attributes": {
    "project_status": "Suspended",
    "name": "Const. of Barangay Hall/Multi-Purpose Hall at Brgy Gatas",
    "code": "2SAIL20-R8000-1C7",
    "location": "[\"7\"]",
    "cost": 1438035.83,
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
    "prop_fund": "17",
    "prop_type": "5",
    "prop_category": "1",
    "prop_sector": "[]",
    "prop_assign": "2",
    "accom_total": 50.2,
    "accom_value": null,
    "accom_date": null,
    "accom_elapse": null,
    "accom_slippage": null,
    "accom_period": null,
    "accom_todate": null,
    "accom_qty": null,
    "accom_amt": null,
    "date": null,
    "status": "CREATED",
    "start": "2023-01-24",
    "target": "2023-04-24",
    "office": "Engineer's Office",
    "assigned": null,
    "appropriation": 1500000,
    "abc": 0,
    "status_id": {
      "data": null
    },
    "remarks": "Suspended/non-moving  due to  the on-going case with filed by ENGR. HERRERA\n",
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
  "id": "601",
  "attributes": {
    "project_status": "Completed",
    "name": "Const of Stage and Bleacher at Brgy Kalasan",
    "code": "2SAIL20-R8000-1C8",
    "location": null,
    "cost": 1147485.03,
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
    "appropriation": 1200000,
    "abc": 0,
    "status_id": {
      "data": null
    },
    "remarks": null,
    "entry_type": "IMPLEMENTED",
    "bid_date": "2021-04-07",
    "year": 2020,
    "abc_published": null,
    "prop_takers": null,
    "prop_infra": null,
    "time_extension": null
  }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
  "id": "602",
  "attributes": {
    "project_status": "Completed",
    "name": "Const of Barangay Stage at Brgy Lison Valley",
    "code": "2SAIL20-R8000-1C9",
    "location": "[\"40\"]",
    "cost": 290462.92,
    "contractor": {
      "data": {
        "id": "45",
        "attributes": {
          "name": "Pyramid Builders",
          "street_purok": null
        }
      }
    },
    "duration": 60,
    "adjusted": null,
    "total": null,
    "weight": null,
    "prop_sdg": "[]",
    "prop_fund": "24",
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
    "start": "2022-02-25",
    "target": "2022-04-26",
    "office": "Engineer's Office",
    "assigned": null,
    "appropriation": 300000,
    "abc": 0,
    "status_id": {
      "data": null
    },
    "remarks": "100%  physically completed based on approved technical plans & specifications\n\nActual Completion Date: 4/26/2022\n\n",
    "entry_type": "IMPLEMENTED",
    "bid_date": "2021-12-08",
    "year": 2020,
    "abc_published": null,
    "prop_takers": null,
    "prop_infra": null,
    "time_extension": null
  }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
  "id": "603",
  "attributes": {
    "project_status": "Completed",
    "name": "Const. of Multi-Purpose Hall at Brgy Muricay",
    "code": "2SAIL20-R8000-1C10",
    "location": "[]",
    "cost": 636718.29,
    "contractor": {
      "data": {
        "id": "49",
        "attributes": {
          "name": "AM Tabayag Construction",
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
    "appropriation": 650000,
    "abc": 0,
    "status_id": {
      "data": null
    },
    "remarks": "\n",
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
  "id": "604",
  "attributes": {
    "project_status": "Completed",
    "name": "Renovation of Multi-Purpose Building (Barangay Hall) at Napolan",
    "code": "2SAIL20-R8000-1C11",
    "location": null,
    "cost": 475390.57,
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
    "bid_date": "2021-05-28",
    "year": 2020,
    "abc_published": null,
    "prop_takers": null,
    "prop_infra": null,
    "time_extension": null
  }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
  "id": "605",
  "attributes": {
    "project_status": "Completed",
    "name": "Const/Impvt of Barangay Hall, San Jose",
    "code": "2SAIL20-R8000-1C12",
    "location": null,
    "cost": 773479.88,
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
    "appropriation": 800000,
    "abc": 0,
    "status_id": {
      "data": null
    },
    "remarks": null,
    "entry_type": "IMPLEMENTED",
    "bid_date": "2021-03-23",
    "year": 2020,
    "abc_published": null,
    "prop_takers": null,
    "prop_infra": null,
    "time_extension": null
  }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
  "id": "606",
  "attributes": {
    "project_status": "Completed",
    "name": "Const of Bleacher at Covered Court, Brgy Tawagan",
    "code": "2SAIL20-R8000-1C13",
    "location": null,
    "cost": 967665.55,
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
  "id": "607",
  "attributes": {
    "project_status": "Completed",
    "name": "Const of Stage at Tiguma E/S",
    "code": "2SAIL20-R8000-1C14",
    "location": null,
    "cost": 287481.43,
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
    "bid_date": null,
    "year": 2020,
    "abc_published": null,
    "prop_takers": null,
    "prop_infra": null,
    "time_extension": null
  }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
  "id": "608",
  "attributes": {
    "project_status": "Completed",
    "name": "Const of Perimeter Fence at Brgy. Hall & health Center, Bomba",
    "code": "2SAIL20-R8000-1D1",
    "location": null,
    "cost": 480075.4,
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
    "bid_date": "2021-12-17",
    "year": 2020,
    "abc_published": null,
    "prop_takers": null,
    "prop_infra": null,
    "time_extension": null
  }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
  "id": "609",
  "attributes": {
    "project_status": "Completed",
    "name": "Stairway Repair at Brgy Dumagoc",
    "code": "2SAIL20-R8000-1D2",
    "location": null,
    "cost": 95126.76,
    "contractor": {
      "data": {
        "id": "50",
        "attributes": {
          "name": "Diplahan Construction",
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
    "appropriation": 100000,
    "abc": 0,
    "status_id": {
      "data": null
    },
    "remarks": null,
    "entry_type": "IMPLEMENTED",
    "bid_date": "2021-04-28",
    "year": 2020,
    "abc_published": null,
    "prop_takers": null,
    "prop_infra": null,
    "time_extension": null
  }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
  "id": "610",
  "attributes": {
    "project_status": "Not Yet Started",
    "name": "Construction of Water Tank & Pipe Installation at Brgy. Padolunan",
    "code": "2SAIL20-R8000-1D3",
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
    "appropriation": 300000,
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
  "id": "611",
  "attributes": {
    "project_status": "Completed",
    "name": "Completion of Solar Dryer & Fence Construction, Brgy Tulangan",
    "code": "2SAIL20-R8000-1D4",
    "location": null,
    "cost": 483015.4,
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
    "bid_date": "2020-11-05",
    "year": 2020,
    "abc_published": null,
    "prop_takers": null,
    "prop_infra": null,
    "time_extension": null
  }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
  "id": "612",
  "attributes": {
    "project_status": "On-Going",
    "name": "Const. of Mini Port terminal at Brgy White Beach",
    "code": "2SAIL20-R8000-1D5",
    "location": null,
    "cost": 86722.26,
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
    "appropriation": 90000,
    "abc": 0,
    "status_id": {
      "data": null
    },
    "remarks": null,
    "entry_type": "IMPLEMENTED",
    "bid_date": "2021-01-21",
    "year": 2020,
    "abc_published": null,
    "prop_takers": null,
    "prop_infra": null,
    "time_extension": null
  }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
  "id": "616",
  "attributes": {
    "project_status": "Completed",
    "name": "Improvement/Landscaping of Plaza Luz",
    "code": "LGSF AC",
    "location": "[\"12\"]",
    "cost": 792579.44,
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
    "prop_fund": "9",
    "prop_type": "11",
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
    "start": "2022-01-13",
    "target": "2022-03-14",
    "office": "Engineer's Office",
    "assigned": null,
    "appropriation": 796152,
    "abc": 0,
    "status_id": {
      "data": null
    },
    "remarks": "100%  physically completed based on approved technical plans & specifications\n\nActual Date Completed: March 10, 2022",
    "entry_type": "IMPLEMENTED",
    "bid_date": "2021-11-24",
    "year": 2020,
    "abc_published": null,
    "prop_takers": "JOMARC",
    "prop_infra": null,
    "time_extension": null
  }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
  "id": "617",
  "attributes": {
    "project_status": "Completed",
    "name": "Conc/Impvt of Brgy Banale Road (After School)",
    "code": "DF20-8000-B1c",
    "location": null,
    "cost": 482991,
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
  "id": "618",
  "attributes": {
    "project_status": "Completed",
    "name": "Conc/Impvt of Kagawasan to Alegria Brgy. Road",
    "code": "DF20-8000-B1e",
    "location": null,
    "cost": 480110.95,
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
    "appropriation": 500000,
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
  "id": "619",
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
  "id": "620",
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
  "id": "621",
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
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
  "id": "622",
  "attributes": {
    "project_status": "Completed",
    "name": "Rehab/Concreting of Road at Lower Sibatang Near Lingue NHS",
    "code": "2SAIP20 - 8000 - C1a9",
    "location": null,
    "cost": 0,
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
    "bid_date": null,
    "year": 2020,
    "abc_published": null,
    "prop_takers": null,
    "prop_infra": null,
    "time_extension": null
  }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
  "id": "623",
  "attributes": {
    "project_status": "On-Going",
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
  "id": "624",
  "attributes": {
    "project_status": "On-Going",
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
  "id": "625",
  "attributes": {
    "project_status": "Bidded",
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
  "id": "627",
  "attributes": {
    "project_status": "Completed",
    "name": "Rehab of Road From Purok Rosas to Prk Bombil, Brgy. Bulawan",
    "code": "2SAIL20-R8000-1A3",
    "location": "[]",
    "cost": 893891.65,
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
  "id": "629",
  "attributes": {
     "project_status": "Not Yet Started",
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
  "id": "630",
  "attributes": {
    "project_status": "Bidded",
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
  "id": "631",
  "attributes": {
    "project_status": "Bidded",
    "name": "Gravelling/Impvt of Roa from Prk talong, brgy Lumad",
    "code": "2SAIL20-R8000-1A16",
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
    "bid_date": "2021-04-30",
    "year": 2020,
    "abc_published": null,
    "prop_takers": null,
    "prop_infra": null,
    "time_extension": null
  }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
  "id": "633",
  "attributes": {
     "project_status": "Completed",
    "name": "Conc/Impvt of Road Leading to Manga Falls",
    "code": "DF20-8000-B7-c",
    "location": null,
    "cost": 5789991.4,
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
  "id": "635",
  "attributes": {
     "project_status": "On-Going",
    "name": "Road Opening/Concreting of Datagan Barangay Road - Kendis Cave to Quarry Site Road",
    "code": "4SAIP20 - 8000R - B1a2",
    "location": null,
    "cost": 0,
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
    "bid_date": null,
    "year": 2020,
    "abc_published": null,
    "prop_takers": null,
    "prop_infra": null,
    "time_extension": null
  }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
  "id": "636",
  "attributes": {
    "project_status": "On-Going",
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
  "id": "638",
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
  "id": "639",
  "attributes": {
    "project_status": "Completed",
    "name": "Improvement of Plaza Luz",
    "code": "5SAIP20 - 8000R - A1e",
    "location": null,
    "cost": 1426822.69,
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
    "appropriation": 1500000,
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
  "id": "640",
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
  "id": "642",
  "attributes": {
    "project_status": "On-Going",
    "name": "ESTABLISHMENT AND DEVELOPMENT OF PAGADIAN CITY WATERPARK",
    "code": "LGSF AC - 2019",
    "location": null,
    "cost": 15907747.04,
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
    "prop_fund": "9",
    "prop_type": null,
    "prop_category": null,
    "prop_sector": null,
    "prop_assign": null,
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
    "appropriation": 15917371,
    "abc": 0,
    "status_id": {
      "data": null
    },
    "remarks": null,
    "entry_type": "IMPLEMENTED",
    "bid_date": "2021-05-17",
    "year": 2020,
    "abc_published": null,
    "prop_takers": null,
    "prop_infra": null,
    "time_extension": null
  }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
  "id": "643",
  "attributes": {
    "project_status": "Completed",
    "name": "POLOYAGAN BEACH GARDEN",
    "code": "LGSF AC - 2020",
    "location": "[\"49\"]",
    "cost": 21698487.82,
    "contractor": {
      "data": {
        "id": "51",
        "attributes": {
          "name": "Jimwen Construction",
          "street_purok": null
        }
      }
    },
    "duration": 360,
    "adjusted": null,
    "total": null,
    "weight": null,
    "prop_sdg": "[]",
    "prop_fund": "9",
    "prop_type": "11",
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
    "start": "2021-07-26",
    "target": "2022-07-21",
    "office": "Engineer's Office",
    "assigned": null,
    "appropriation": 21704010,
    "abc": 0,
    "status_id": {
      "data": null
    },
    "remarks": "\"* 100% physically completed base on approved plans & specifications with time extension based on approved suspension order\n* Adjusted Target Date of Completion: April 25, 2024 as per Variation Order No. 1 & 2\n* Actual Completion Date: April  25,2024\"\n",
    "entry_type": "IMPLEMENTED",
    "bid_date": "2021-05-17",
    "year": 2020,
    "abc_published": null,
    "prop_takers": "JIMWEN CONSTRUCTION",
    "prop_infra": null,
    "time_extension": null
  }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
  "id": "644",
  "attributes": {
    "project_status": "Completed",
    "name": "CONSTRUCTION OF MULTI-PURPOSE BUILDING ( Boulevard-BARKO)",
    "code": "LGSF-FA TO LGUS 2020",
    "location": "[\"13\"]",
    "cost": 14995363.66,
    "contractor": {
      "data": {
        "id": "29",
        "attributes": {
          "name": "Genetian Builders",
          "street_purok": null
        }
      }
    },
    "duration": 360,
    "adjusted": null,
    "total": null,
    "weight": null,
    "prop_sdg": "[]",
    "prop_fund": "9",
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
    "start": "2024-09-16",
    "target": "2025-09-11",
    "office": "Engineer's Office",
    "assigned": null,
    "appropriation": 14999900,
    "abc": 0,
    "status_id": {
      "data": null
    },
    "remarks": "Actual Date Completed: April 19, 2024 as per SWA",
    "entry_type": "IMPLEMENTED",
    "bid_date": "2021-05-17",
    "year": 2020,
    "abc_published": null,
    "prop_takers": null,
    "prop_infra": null,
    "time_extension": null
  }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
  "id": "645",
  "attributes": {
    "project_status": "Completed",
    "name": "Concreting of Alley at Puroks Bagong Buhay, Kapalaran and Tugas, Balangasan\n",
    "code": "2SAILP21 - 8000R - A1A2",
    "location": null,
    "cost": 481655.92,
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
    "appropriation": 500000,
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
  "id": "646",
  "attributes": {
    "project_status": "Not Yet Started",
    "name": "Opening of Road/Concreting from Prk Bagong Silang to Villa Hermosa (Bgry Banale), Balangasan\n",
    "code": "2SAILP21 - 8000R - A1A3",
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
    "appropriation": 800000,
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
  "id": "647",
  "attributes": {
    "project_status": "Bidded",
    "name": "Completion of Road Concreting at Prk Sili to Barangay  Palpalan, Balintawak\n",
    "code": "2SAILP21 - 8000R - A1A4",
    "location": null,
    "cost": 768982.6,
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
    "appropriation": 800000,
    "abc": 0,
    "status_id": {
      "data": null
    },
    "remarks": null,
    "entry_type": "IMPLEMENTED",
    "bid_date": "2023-03-29",
    "year": 2021,
    "abc_published": null,
    "prop_takers": null,
    "prop_infra": null,
    "time_extension": null
  }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
  "id": "648",
  "attributes": {
    "project_status": "Completed",
    "name": "Road Widening from Purok Tinangkong to Purok Sili 3 (One Side Only) Balintawak",
    "code": "2SAILP21-8000R-A1A5",
    "location": "[\"2\"]",
    "cost": 673853,
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
    "prop_sdg": "[]",
    "prop_fund": "17",
    "prop_type": "",
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
    "start": "2022-03-07",
    "target": "2022-04-21",
    "office": "Engineer's Office",
    "assigned": null,
    "appropriation": 700000,
    "abc": 0,
    "status_id": {
      "data": null
    },
    "remarks": "100% physically completed based on approved technical plans & specifications\n\nActual Completion Date: 4/28/2022",
    "entry_type": "IMPLEMENTED",
    "bid_date": "2022-02-15",
    "year": 2021,
    "abc_published": null,
    "prop_takers": "rsq",
    "prop_infra": null,
    "time_extension": null
  }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
  "id": "649",
  "attributes": {
    "project_status": "Completed",
    "name": "Concreting of Road at Barangay Bogo (Leading to Iglesia Ni Kristo Church), Bogo\n",
    "code": "2SAILP21 - 8000R - A1A6",
    "location": null,
    "cost": 1152860.48,
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
    "bid_date": "2022-02-09",
    "year": 2021,
    "abc_published": null,
    "prop_takers": null,
    "prop_infra": null,
    "time_extension": null
  }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
  "id": "650",
  "attributes": {
    "project_status": "Completed",
    "name": "Concreting of Road at Nazareth Area (Prk San Francisco & Prk Hillside), Buenavista",
    "code": "2SAILP21 - 8000R - A1A7",
    "location": null,
    "cost": 770147.21,
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
    "appropriation": 800000,
    "abc": 0,
    "status_id": {
      "data": null
    },
    "remarks": null,
    "entry_type": "IMPLEMENTED",
    "bid_date": "2022-02-22",
    "year": 2021,
    "abc_published": null,
    "prop_takers": null,
    "prop_infra": null,
    "time_extension": null
  }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
  "id": "651",
  "attributes": {
    "project_status": "Completed",
    "name": "Widening of Bulatok Barangay Road",
    "code": "2SAILP21 - 8000R - A1A8",
    "location": null,
    "cost": 238875.71,
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
    "appropriation": 250000,
    "abc": 0,
    "status_id": {
      "data": null
    },
    "remarks": null,
    "entry_type": "IMPLEMENTED",
    "bid_date": "2022-01-18",
    "year": 2021,
    "abc_published": null,
    "prop_takers": null,
    "prop_infra": null,
    "time_extension": null
  }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
  "id": "652",
  "attributes": {
     "project_status": "Completed",
    "name": " Road Improvement at Zone 7 (Fronting IBT), Bulatok",
    "code": "2SAILP21 - 8000R - A1A9",
    "location": null,
    "cost": 497720.85,
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
    "appropriation": 500000,
    "abc": 0,
    "status_id": {
      "data": null
    },
    "remarks": null,
    "entry_type": "IMPLEMENTED",
    "bid_date": "2022-01-18",
    "year": 2021,
    "abc_published": null,
    "prop_takers": null,
    "prop_infra": null,
    "time_extension": null
  }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
  "id": "653",
  "attributes": {
    "project_status": "Bidded",
    "name": "Improvement/Opening of Road at Prk Bombil (Difficult Portion), Bulawan",
    "code": "2SAILP21 - 8000R - A1A10",
    "location": null,
    "cost": 962327.59,
    "contractor": {
      "data": {
        "id": "52",
        "attributes": {
          "name": "Ammara Construction",
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
    "bid_date": "2022-09-08",
    "year": 2021,
    "abc_published": null,
    "prop_takers": null,
    "prop_infra": null,
    "time_extension": null
  }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
  "id": "654",
  "attributes": {
    "project_status": "Completed",
    "name": "Concreting of Road From Prk Fatima to Boundary Brgy Dao",
    "code": "2SAILP21 - 8000R - A1A11",
    "location": null,
    "cost": 963741.7,
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
    "appropriation": 1000000,
    "abc": 0,
    "status_id": {
      "data": null
    },
    "remarks": null,
    "entry_type": "IMPLEMENTED",
    "bid_date": "2022-02-01",
    "year": 2021,
    "abc_published": null,
    "prop_takers": null,
    "prop_infra": null,
    "time_extension": null
  }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
  "id": "655",
  "attributes": {
    "project_status": "Completed",
    "name": "Concreting of Ditoray Barangay Road to Deborok Barangay Road (Difficult Portion), Ditoray",
    "code": "2SAILP21 - 8000R - A1A12",
    "location": null,
    "cost": 960498,
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
    "appropriation": 1000000,
    "abc": 0,
    "status_id": {
      "data": null
    },
    "remarks": null,
    "entry_type": "IMPLEMENTED",
    "bid_date": "2022-01-25",
    "year": 2021,
    "abc_published": null,
    "prop_takers": null,
    "prop_infra": null,
    "time_extension": null
  }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
  "id": "656",
  "attributes": {
    "project_status": "Completed",
    "name": "Concreting of Alley (Besdie Sy Luy - End of Ps Bank) Gatas",
    "code": "2SAILP21 - 8000R - A1A13",
    "location": null,
    "cost": 619995,
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
    "appropriation": 650000,
    "abc": 0,
    "status_id": {
      "data": null
    },
    "remarks": null,
    "entry_type": "IMPLEMENTED",
    "bid_date": "2022-03-18",
    "year": 2021,
    "abc_published": null,
    "prop_takers": null,
    "prop_infra": null,
    "time_extension": null
  }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
  "id": "657",
  "attributes": {
    "project_status": "Completed",
    "name": "Concreting of Road Leading to E/S (Inc. Reblocking Fronting Barangay Hall) Gubac",
    "code": "2SAILP21-8000R-A1A14",
    "location": "[\"31\"]",
    "cost": 959500,
    "contractor": {
      "data": {
        "id": "1",
        "attributes": {
          "name": "CTG Construction",
          "street_purok": "Purok 1"
        }
      }
    },
    "duration": 50,
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
    "start": "2022-03-17",
    "target": "2022-05-06",
    "office": "Engineer's Office",
    "assigned": null,
    "appropriation": 1000000,
    "abc": 0,
    "status_id": {
      "data": null
    },
    "remarks": "\"100% physically completed based on approved technical plans & specifications\n\nw/ time extension as per approved Suspension  Order No.1\"\n\nActual Date Completed: 5/7/2022",
    "entry_type": "IMPLEMENTED",
    "bid_date": "2022-02-15",
    "year": 2021,
    "abc_published": null,
    "prop_takers": "CTG-HERME GUINEA",
    "prop_infra": null,
    "time_extension": null
  }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
  "id": "658",
  "attributes": {
    "project_status": "Completed",
    "name": "Road Improvement at Prk Bologon, Gubang",
    "code": "2SAILP21 - 8000R - A1A15",
    "location": null,
    "cost": 1356968.5,
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
    "appropriation": 1400000,
    "abc": 0,
    "status_id": {
      "data": null
    },
    "remarks": null,
    "entry_type": "IMPLEMENTED",
    "bid_date": "2022-11-14",
    "year": 2021,
    "abc_published": null,
    "prop_takers": null,
    "prop_infra": null,
    "time_extension": null
  }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
  "id": "659",
  "attributes": {
    "project_status": "On-Going",
    "name": "Concreting of Road at Purok Waling2 (Difficult Portion) Kagawasan",
    "code": "2SAILP21-8000R-A1A16",
    "location": "[\"33\"]",
    "cost": 778338.68,
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
    "prop_fund": "17",
    "prop_type": "1",
    "prop_category": "2",
    "prop_sector": "[]",
    "prop_assign": "2",
    "accom_total": 15,
    "accom_value": null,
    "accom_date": null,
    "accom_elapse": null,
    "accom_slippage": null,
    "accom_period": null,
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
    "abc": 778338.68,
    "status_id": {
      "data": null
    },
    "remarks": "On-going implementation of the project\n",
    "entry_type": "IMPLEMENTED",
    "bid_date": "2021-03-17",
    "year": 2021,
    "abc_published": null,
    "prop_takers": "LYNETTE PERALTA",
    "prop_infra": null,
    "time_extension": null
  }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
  "id": "660",
  "attributes": {
    "project_status": "Completed",
    "name": "Concreting of Road from Prk Talong II to Prk Balatong 1 (Difficult Portion) Kahayagan",
    "code": "2SAILP21 - 8000R - A1A17",
    "location": null,
    "cost": 871892,
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
    "appropriation": 900000,
    "abc": 0,
    "status_id": {
      "data": null
    },
    "remarks": null,
    "entry_type": "IMPLEMENTED",
    "bid_date": "2022-02-22",
    "year": 2021,
    "abc_published": null,
    "prop_takers": null,
    "prop_infra": null,
    "time_extension": null
  }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
  "id": "661",
  "attributes": {
    "project_status": "Completed",
    "name": "Concreting of Road at Muslim Village, Kawit",
    "code": "2SAILP21 - 8000R - A1A18",
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
    "duration": 45,
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
    "start": "2022-03-04",
    "target": "2022-04-18",
    "office": "Engineer's Office",
    "assigned": null,
    "appropriation": 500000,
    "abc": 0,
    "status_id": {
      "data": null
    },
    "remarks": "100% physically completed based on approved technical plans & specifications\n\nActual Completion Date: 4/15/2022",
    "entry_type": "IMPLEMENTED",
    "bid_date": "2022-01-18",
    "year": 2021,
    "abc_published": null,
    "prop_takers": "SADAM DIMASAR",
    "prop_infra": null,
    "time_extension": null
  }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
  "id": "662",
  "attributes": {
    "project_status": "On-Going",
    "name": "Rehabilitation of San Pedro Bridge Approach (Kawit Side)",
    "code": "2SAILP21-8000R-A1A19",
    "location": "[\"13\"]",
    "cost": 575620,
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
    "prop_fund": "6",
    "prop_type": "2",
    "prop_category": "3",
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
    "appropriation": 600000,
    "abc": 576089.79,
    "status_id": {
      "data": null
    },
    "remarks": "On-going implementation of the project with 35% physical accomplishment as of Dec.6, 2024",
    "entry_type": "IMPLEMENTED",
    "bid_date": "2022-11-03",
    "year": 2021,
    "abc_published": null,
    "prop_takers": null,
    "prop_infra": null,
    "time_extension": null
  }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
  "id": "663",
  "attributes": {
    "project_status": "Completed",
    "name": "Concreting of Road From Prk Pinya to Prk Santol, La Suerte",
    "code": "2SAILP21 - 8000R - A1A20",
    "location": null,
    "cost": 769519,
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
    "appropriation": 800000,
    "abc": 0,
    "status_id": {
      "data": null
    },
    "remarks": null,
    "entry_type": "IMPLEMENTED",
    "bid_date": "2022-09-06",
    "year": 2021,
    "abc_published": null,
    "prop_takers": null,
    "prop_infra": null,
    "time_extension": null
  }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
  "id": "664",
  "attributes": {
     "project_status": "Completed",
    "name": "Concreting of Road at Three (3) Puroks Up to Lumad Boundary (At 600T Each), Lala",
    "code": "2SAILP21 - 8000R - A1A21",
    "location": null,
    "cost": 1742997,
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
    "appropriation": 1800000,
    "abc": 0,
    "status_id": {
      "data": null
    },
    "remarks": null,
    "entry_type": "IMPLEMENTED",
    "bid_date": "2022-02-09",
    "year": 2021,
    "abc_published": null,
    "prop_takers": null,
    "prop_infra": null,
    "time_extension": null
  }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
  "id": "665",
  "attributes": {
    "project_status": "Completed",
    "name": "Concreting of Alley at Prk Masinugtanon, Lala",
    "code": "2SAILP21 - 8000R - A1A22",
    "location": null,
    "cost": 386090.85,
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
    "appropriation": 400000,
    "abc": 0,
    "status_id": {
      "data": null
    },
    "remarks": null,
    "entry_type": "IMPLEMENTED",
    "bid_date": "2022-02-01",
    "year": 2021,
    "abc_published": null,
    "prop_takers": null,
    "prop_infra": null,
    "time_extension": null
  }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
  "id": "666",
  "attributes": {
    "project_status": "Bidded",
    "name": "Concreting of Road From Prk Sandayong to prk Tres (Inc RCCP Installation Fronting IFFFC Church), Lapedian",
    "code": "2SAILP21 - 8000R - A1A23",
    "location": null,
    "cost": 578609,
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
    "appropriation": 600000,
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
  "id": "667",
  "attributes": {
    "project_status": "Completed",
    "name": "Continuation of Road Concreting at Purok Nangka - Bulatok, Lenienza",
    "code": "2SAILP21 - 8000R - A1A24",
    "location": "[\"39\"]",
    "cost": 577745,
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
    "bid_date": "2022-09-12",
    "year": 2021,
    "abc_published": null,
    "prop_takers": null,
    "prop_infra": null,
    "time_extension": null
  }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
  "id": "668",
  "attributes": {
    "project_status": "Completed",
    "name": "Concreting of Road From Prk Kamansi - Alta Tierra, Lenienza",
    "code": "2SAILP21 - 8000R - A1A25",
    "location": "[]",
    "cost": 578421,
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
    "bid_date": "2022-09-06",
    "year": 2021,
    "abc_published": null,
    "prop_takers": null,
    "prop_infra": null,
    "time_extension": null
  }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
  "id": "669",
  "attributes": {
    "project_status": "Completed",
    "name": "Concreting of Road at Purok Manga, Lenienza",
    "code": "2SAILP21 - 8000R - A1A26",
    "location": "[\"39\"]",
    "cost": 578855,
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
    "bid_date": "2022-09-06",
    "year": 2021,
    "abc_published": null,
    "prop_takers": null,
    "prop_infra": null,
    "time_extension": null
  }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
  "id": "670",
  "attributes": {
     "project_status": "Bidded",
    "name": "Concreting of Road at Purok Sirawag, lison Valley",
    "code": "2SAILP21 - 8000R - A1A27",
    "location": null,
    "cost": 725995.3,
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
    "appropriation": 750000,
    "abc": 0,
    "status_id": {
      "data": null
    },
    "remarks": null,
    "entry_type": "IMPLEMENTED",
    "bid_date": "2022-09-08",
    "year": 2021,
    "abc_published": null,
    "prop_takers": null,
    "prop_infra": null,
    "time_extension": null
  }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
  "id": "671",
  "attributes": {
    "project_status": "Completed",
    "name": "Concreting of Road From Market to E/S (Half Lane), Lourdes",
    "code": "2SAILP21 - 8000R - A1A28",
    "location": null,
    "cost": 964402.5,
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
    "bid_date": "2022-09-08",
    "year": 2021,
    "abc_published": null,
    "prop_takers": null,
    "prop_infra": null,
    "time_extension": null
  }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
  "id": "672",
  "attributes": {
    "project_status": "Bidded",
    "name": "Conc/Reblocking of Road at Purok Santol (Near Lingue H/S) Lower Sibatang",
    "code": "2SAILP21 - 8000R - A1A29",
    "location": null,
    "cost": 774638.6,
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
    "appropriation": 800000,
    "abc": 0,
    "status_id": {
      "data": null
    },
    "remarks": null,
    "entry_type": "IMPLEMENTED",
    "bid_date": "2022-09-06",
    "year": 2021,
    "abc_published": null,
    "prop_takers": null,
    "prop_infra": null,
    "time_extension": null
  }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
  "id": "673",
  "attributes": {
    "project_status": "Completed",
    "name": "Concreting of Road From Brgy Proper (1/2 Lane) to Lala Boundary, Lumad",
    "code": "2SAILP21 - 8000R - A1A30",
    "location": null,
    "cost": 967962,
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
    "bid_date": "2022-01-18",
    "year": 2021,
    "abc_published": null,
    "prop_takers": null,
    "prop_infra": null,
    "time_extension": null
  }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
  "id": "674",
  "attributes": {
    "project_status": "Completed",
    "name": "Rehab of Boulevard at Prk. Fisherville & Prk Alindahaw, Lumbia",
    "code": "2SAILP21 - 8000R - A1A31",
    "location": "[]",
    "cost": 1443263.56,
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
    "prop_fund": "17",
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
    "bid_date": "2023-02-06",
    "year": 2021,
    "abc_published": null,
    "prop_takers": null,
    "prop_infra": null,
    "time_extension": null
  }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
  "id": "675",
  "attributes": {
    "project_status": "Completed",
    "name": "Concreting of Road from Lower Lumbia to Prk Alindahaw",
    "code": "2SAILP21 - 8000R - A1A32",
    "location": null,
    "cost": 575589,
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
    "appropriation": 575589,
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
  "id": "676",
  "attributes": {
    "project_status": "Completed",
    "name": "Concreting of Road at Prk Palmera, Napolan",
    "code": "2SAILP21 - 8000R - A1A33",
    "location": null,
    "cost": 481299,
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
    "appropriation": 500000,
    "abc": 0,
    "status_id": {
      "data": null
    },
    "remarks": null,
    "entry_type": "IMPLEMENTED",
    "bid_date": "2022-09-12",
    "year": 2021,
    "abc_published": null,
    "prop_takers": null,
    "prop_infra": null,
    "time_extension": null
  }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
  "id": "677",
  "attributes": {
    "project_status": "Completed",
    "name": "Concreting of Road at Prk Makiangayon, Napolan",
    "code": "2SAILP21 - 8000R - A1A34",
    "location": null,
    "cost": 479577,
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
    "appropriation": 500000,
    "abc": 0,
    "status_id": {
      "data": null
    },
    "remarks": null,
    "entry_type": "IMPLEMENTED",
    "bid_date": "2022-09-12",
    "year": 2021,
    "abc_published": null,
    "prop_takers": null,
    "prop_infra": null,
    "time_extension": null
  }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
  "id": "678",
  "attributes": {
    "project_status": "Bidded",
    "name": "Opening/Concreting of Road Along Sabate St (Near Provincial Jail), San Jose",
    "code": "2SAILP21 - 8000R - A1A35",
    "location": null,
    "cost": 964899.8,
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
    "bid_date": "2023-02-27",
    "year": 2021,
    "abc_published": null,
    "prop_takers": null,
    "prop_infra": null,
    "time_extension": null
  }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
  "id": "679",
  "attributes": {
    "project_status": "Completed",
    "name": "Concreting of Alley at Prk Bougainvilla, San Pedro",
    "code": "2SAILP21-8000R-A1A36",
    "location": "[\"13\"]",
    "cost": 977891.97,
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
    "start": "2024-10-30",
    "target": "2024-12-14",
    "office": "Engineer's Office",
    "assigned": null,
    "appropriation": 1000000,
    "abc": 978891.94,
    "status_id": {
      "data": null
    },
    "remarks": "\"* 100% physically completed base on approved technical plans & specifications\n* w/ time extension as per approved Suspension Order No. 1\n* Adjusted Targe Date of Completion: Dec. 23, 2023\"\n* Actual Date Completed:12/23/2023",
    "entry_type": "IMPLEMENTED",
    "bid_date": null,
    "year": 2021,
    "abc_published": null,
    "prop_takers": "CAPT. SUMAMPONG",
    "prop_infra": null,
    "time_extension": null
  }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
  "id": "680",
  "attributes": {
    "project_status": "Not Yet Started",
    "name": "Concreting of Alley at Purok Paglaum, Sta. Lucia",
    "code": "2SAILP21 - 8000R - A1A37",
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
    "prop_fund": "17",
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
    "appropriation": 500000,
    "abc": 0,
    "status_id": {
      "data": null
    },
    "remarks": "Bidded but not yet started",
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
  "id": "681",
  "attributes": {
     "project_status": "Completed",
    "name": "Concreting of Road at Salera St. (jct Pulmones St - Dablo St) Sta. Lucia",
    "code": "2SAILP21 - 8000R - A1A38",
    "location": null,
    "cost": 482437.48,
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
    "bid_date": "2022-03-17",
    "year": 2021,
    "abc_published": null,
    "prop_takers": null,
    "prop_infra": null,
    "time_extension": null
  }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
  "id": "682",
  "attributes": {
     "project_status": "Completed",
    "name": "Opening of Alley at Prk Makiangayon (Beside Luis) Jct Duterte - Datoc St, Sta Lucia",
    "code": "2SAILP21-8000R-A1A39",
    "location": "[\"14\"]",
    "cost": 482723.5,
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
    "prop_fund": "17",
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
    "target": null,
    "office": "Engineer's Office",
    "assigned": null,
    "appropriation": 500000,
    "abc": 483222.59,
    "status_id": {
      "data": null
    },
    "remarks": null,
    "entry_type": "IMPLEMENTED",
    "bid_date": "2022-12-15",
    "year": 2021,
    "abc_published": null,
    "prop_takers": "Genevive Malalis",
    "prop_infra": null,
    "time_extension": null
  }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
  "id": "683",
  "attributes": {
    "project_status": "Completed",
    "name": "Opening of Alley at Purok Madasigon, (Jct Natl Highway - Jamisola St), Sta. Lucia",
    "code": "2SAILP21 - 8000R - A1A40",
    "location": "[]",
    "cost": 481311.75,
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
    "bid_date": "2022-02-23",
    "year": 2021,
    "abc_published": null,
    "prop_takers": null,
    "prop_infra": null,
    "time_extension": null
  }
}), db_connection).await,
convert_projects_into_active_model_v2(&json!({
  "id": "684",
  "attributes": {
    "project_status": "Completed",
    "name": "Concreting of Alley at Prk Spring and Prk Malipayon (Beside Buffalo) Sta. Lucia",
    "code": "2SAILP21 - 8000R - A1A41",
    "location": null,
    "cost": 480075.11,
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
    "bid_date": "2022-03-17",
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