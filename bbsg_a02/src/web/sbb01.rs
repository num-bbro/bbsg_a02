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

use crate::dcl::VarType;
use crate::dcl::DNM;
use crate::dcl::FIR_LIST;
use crate::dcl::SSHOW_YEAR_BEG;
use crate::dcl::SSHOW_YEAR_END;
use crate::p08::ld_sub_info;
use crate::p08::SubInfo;
use std::collections::HashMap;

/*
const FLD_LIST: [(VarType, &str); 14] = [
    (VarType::FirBilAccu, ""),
    (VarType::FirCashFlow, "/tr01"),
    (VarType::FirDRSave, ""),
    (VarType::FirBatSubSave, ""),
    (VarType::FirBatSvgSave, ""),
    (VarType::FirBatPriceDiff, ""),
    (VarType::FirMetBoxSave, ""),
    (VarType::FirLaborSave, "/tr02"),
    (VarType::FirMetSell, "/tr02"),
    (VarType::FirEMetSave, ""),
    (VarType::FirMetReadSave, ""),
    (VarType::FirMetDisSave, ""),
    (VarType::FirTouSell, ""),
    (VarType::FirTouReadSave, ""),
];
*/

#[derive(Template, WebTemplate, Debug, Default)]
#[template(path = "sbb01.html")]
pub struct WebTemp {
    name: String,
    assv: Vec<PeaAssVar>,
    sbif: HashMap<String, SubInfo>,
    //flds: Vec<(VarType, &'static str)>,
    se_fld: VarType,
}

pub async fn sbb01(para: Query<Param>) -> WebTemp {
    let mut fldm = HashMap::<String, VarType>::new();
    for vt in &FIR_LIST {
        let fd = format!("{:?}", vt);
        fldm.insert(fd, vt.clone());
    }
    let fld = if let Some(fld) = &para.fld {
        fld.clone()
    } else {
        format!("{:?}", FIR_LIST[0])
    };
    let Some(se_fld) = fldm.get(&fld) else {
        println!("NO SELECTED FIELD");
        return WebTemp::default();
    };
    // ==============
    /*
    let Some(ref sbid) = para.sbid else {
        println!("NO SBID");
        return WebTemp::default();
    };
    */
    // ==== read rw3 data
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
        name: "Substation - sb01 (sort by sub)".to_string(),
        assv,
        sbif: sbif.clone(),
        //flds: FLD_LIST.to_vec(),
        se_fld: se_fld.clone(),
    }
}
