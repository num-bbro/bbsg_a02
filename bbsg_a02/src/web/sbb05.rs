use crate::dcl::Pan;
use crate::dcl::PeaAssVar;
use askama::Template;
use askama_web::WebTemplate;
use axum::extract::Query;
use serde::Deserialize;

#[derive(Debug, Deserialize, Default)]
pub struct Param {
    pub fld: Option<String>,
}

use crate::dcl::DNM;
use crate::dcl::SHOW_FLDS3;
use crate::p08::ld_sub_info;
use crate::p08::SubInfo;
use std::collections::HashMap;

#[derive(Template, WebTemplate, Debug, Default)]
#[template(path = "sbb05.html")]
pub struct WebTemp {
    name: String,
    assv: Vec<PeaAssVar>,
    sbif: HashMap<String, SubInfo>,
}

pub async fn sbb05(_para: Query<Param>) -> WebTemp {
    let name = "SUBSTATION".to_string();
    let Ok(buf) = std::fs::read(format!("{DNM}/000-sbrw.bin")) else {
        println!("NO rw3.bin file:");
        return WebTemp::default();
    };
    // ==== read rw3 data
    let Ok((assv, _)): Result<(Vec<PeaAssVar>, usize), _> =
        bincode::decode_from_slice(&buf[..], bincode::config::standard())
    else {
        println!("Failed to decode rw3:");
        return WebTemp::default();
    };
    //let sbif = sub_inf(); //HashMap<String, SubstInfo>
    let sbif = ld_sub_info();
    WebTemp {
        name,
        assv,
        sbif: sbif.clone(),
        //flds: FLD_LIST.to_vec(),
        //se_fld: se_fld.clone(),
    }
}
