use azd::azd_info;
use mylogger::info;
fn main() {
    mylogger::init();
    info!("from main");
    azd_info();
}
