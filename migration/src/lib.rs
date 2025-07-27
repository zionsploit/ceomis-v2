#![recursion_limit = "512"]


pub use sea_orm_migration::prelude::*;
pub struct Migrator;

mod helpers;

mod m20250529_025602_create_user_account_table;
mod m20250531_071243_create_user_roles_table;
mod m20250531_071544_seed_users_roles_table;
mod m20250531_081246_alter_user_table_add_user_role;
mod m20250601_055837_create_users_info_table;
mod m20250604_020720_create_settings_sdg_table;
mod m20250604_084258_create_settings_sof_table;
mod m20250606_054120_create_settings_type_table;
mod m20250606_140927_create_settings_incharge_table;
mod m20250606_151143_create_settings_categories_table;
mod m20250606_161035_create_settings_sector_table;
mod m20250608_050646_create_settings_barangay_table;
mod m20250608_133113_create_settings_takers_table;
mod m20250610_101230_seed_system_admin_variant_in_user_roles_table;
mod m20250612_095940_alter_add_is_delete_in_users_table;
mod m20250612_152942_create_contractors_table;
mod m20250612_155550_seed_contractors_data;
mod m20250612_170355_alter_add_is_delete_column_in_contractors_table;
mod m20250614_060710_create_project_status_enum_table;
mod m20250617_035855_create_project_table;
mod m20250620_181759_alter_new_column_is_disposed_in_projects_table;
mod m20250629_112258_create_projects_infra_code_table;
mod m20250630_091430_create_projects_remarks_table;
mod m20250630_091955_create_projects_remarks_img_table;
mod m20250630_092943_alter_add_original_file_name_column_in_project_remarks_img_table;
mod m20250701_135425_alter_add_project_id_column_in_projects_monitoring_remarks_table;
mod m20250701_145702_create_projects_payment_table;
mod m20250720_083706_seed_initial_admin_users;
mod m20250723_122329_seed_project_goals_data;
mod m20250723_132547_seed_project_sof_data;
mod m20250723_133041_seed_project_type_data;
mod m20250723_133353_seed_project_incharge_data;
mod m20250723_145751_seed_project_categories_data;
mod m20250723_150102_seed_project_sectors_data;
mod m20250723_150945_seed_project_barangays_data;
mod m20250723_152544_seed_project_takers_data;
mod m20250724_101855_seed_50_projects_data;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250529_025602_create_user_account_table::Migration),
            Box::new(m20250531_071243_create_user_roles_table::Migration),
            Box::new(m20250531_071544_seed_users_roles_table::Migration),
            Box::new(m20250531_081246_alter_user_table_add_user_role::Migration),
            Box::new(m20250601_055837_create_users_info_table::Migration),
            Box::new(m20250604_020720_create_settings_sdg_table::Migration),
            Box::new(m20250604_084258_create_settings_sof_table::Migration),
            Box::new(m20250606_054120_create_settings_type_table::Migration),
            Box::new(m20250606_140927_create_settings_incharge_table::Migration),
            Box::new(m20250606_151143_create_settings_categories_table::Migration),
            Box::new(m20250606_161035_create_settings_sector_table::Migration),
            Box::new(m20250608_050646_create_settings_barangay_table::Migration),
            Box::new(m20250608_133113_create_settings_takers_table::Migration),
            Box::new(m20250610_101230_seed_system_admin_variant_in_user_roles_table::Migration),
            Box::new(m20250612_095940_alter_add_is_delete_in_users_table::Migration),
            Box::new(m20250612_152942_create_contractors_table::Migration),
            Box::new(m20250612_155550_seed_contractors_data::Migration),
            Box::new(m20250612_170355_alter_add_is_delete_column_in_contractors_table::Migration),
            Box::new(m20250614_060710_create_project_status_enum_table::Migration),
            Box::new(m20250617_035855_create_project_table::Migration),
            Box::new(m20250620_181759_alter_new_column_is_disposed_in_projects_table::Migration),
            Box::new(m20250629_112258_create_projects_infra_code_table::Migration),
            Box::new(m20250630_091430_create_projects_remarks_table::Migration),
            Box::new(m20250630_091955_create_projects_remarks_img_table::Migration),
            Box::new(m20250630_092943_alter_add_original_file_name_column_in_project_remarks_img_table::Migration),
            Box::new(m20250701_135425_alter_add_project_id_column_in_projects_monitoring_remarks_table::Migration),
            Box::new(m20250701_145702_create_projects_payment_table::Migration),
            Box::new(m20250720_083706_seed_initial_admin_users::Migration),
            Box::new(m20250723_122329_seed_project_goals_data::Migration),
            Box::new(m20250723_132547_seed_project_sof_data::Migration),
            Box::new(m20250723_133041_seed_project_type_data::Migration),
            Box::new(m20250723_133353_seed_project_incharge_data::Migration),
            Box::new(m20250723_145751_seed_project_categories_data::Migration),
            Box::new(m20250723_150102_seed_project_sectors_data::Migration),
            Box::new(m20250723_150945_seed_project_barangays_data::Migration),
            Box::new(m20250723_152544_seed_project_takers_data::Migration),
            Box::new(m20250724_101855_seed_50_projects_data::Migration),
        ]
    }
}
