use crate::dcl::*;
use crate::utl::p01_chk;
use crate::utl::*;
use crate::wrt::write_trn_ass_02;
use std::collections::HashMap;
use std::error::Error;

use sglib03::prc4::SubBenInfo;
use sglib04::prc41::SubCalc;
use sglib04::web1::ben_bill_accu;
use sglib04::web1::ben_boxline_save;
use sglib04::web1::ben_emeter;
//use sglib04::web1::ben_model_entry;
use crate::p08::ld_sub_calc;
use num::pow::Pow;
use sglib04::web1::ben_amt_proj;
use sglib04::web1::ben_cash_flow;
use sglib04::web1::ben_dr_save;
use sglib04::web1::ben_mt_disconn;
use sglib04::web1::ben_mt_read;
use sglib04::web1::ben_outage_labor;
use sglib04::web1::ben_reduce_complain;
use sglib04::web1::ben_sell_meter;
use sglib04::web1::ben_tou_read;
use sglib04::web1::ben_tou_sell;
use sglib04::web1::ben_tou_update;
use sglib04::web1::ben_work_save;
use sglib04::web1::BenProj;

use sglib04::web1::M1P_COST;
use sglib04::web1::M3P_COST;
use sglib04::web1::OP_YEAR_END;
use sglib04::web1::OP_YEAR_START;
use sglib04::web1::TRX_COST;

pub const CALL_CENTER_COST_UP: f32 = 0.04f32;
pub const ASSET_WORTH_RATIO: f32 = 0.2f32;
pub const MODEL_ENTRY_RATIO: f32 = 0.05f32;
pub const MODEL_ENTRY_COST: f32 = 1000f32;

/// ประมวลผลรวมเพื่อเกณฑ์การคัดเลือก
/// summery transformaters to substation
pub fn stage_03() -> Result<(), Box<dyn Error>> {
    let buf = std::fs::read(format!("{DNM}/000_pea.bin")).unwrap();
    let (pea, _): (Pea, usize) =
        bincode::decode_from_slice(&buf[..], bincode::config::standard()).unwrap();
    let mut aids: Vec<_> = pea.aream.keys().collect();
    aids.sort();
    //println!("..1");
    let subhs = p01_chk();
    //println!("..1.1");
    //let sbtr = ld_sb_tr0();
    let sbtr = ld_sub_calc();
    //println!("..1.2");
    //println!("sbtr: {}", sbtr.len());
    let mut emp = Vec::<(u32, f32)>::new();
    for y in OP_YEAR_START..=OP_YEAR_END {
        emp.push((y, 0f32));
    }
    //
    //let mut pvcn = 0;
    //println!("..2");
    let mut v_pvas = Vec::<PeaAssVar>::new();
    let mut v_sbas = Vec::<PeaAssVar>::new();
    //let mut sbas_mx = PeaAssVar::default();
    let mut sbas_mx = PeaAssVar::from(0u64);
    for aid in aids {
        //println!("..3");
        let Some(ar) = pea.aream.get(aid) else {
            continue;
        };
        //println!("..4");
        let mut pids: Vec<_> = ar.provm.keys().collect();
        pids.sort();
        for pid in pids {
            //println!("..5");
            let Some(prov) = ar.provm.get(pid) else {
                continue;
            };
            //println!("..6");
            let mut pvas = PeaAssVar::from(0u64);
            pvas.arid = aid.to_string();
            pvas.pvid = pid.to_string();
            println!("  pv:{pid}");
            let mut sids: Vec<_> = prov.subm.keys().collect();
            sids.sort();
            for sid in sids {
                let Some(_sb) = prov.subm.get(sid) else {
                    continue;
                };
                // --- sub
                let Ok(buf) = std::fs::read(format!("{DNM}/{sid}.bin")) else {
                    //println!("PEA {sid} sub load error");
                    continue;
                };
                let (sb, _): (PeaSub, usize) =
                    bincode::decode_from_slice(&buf[..], bincode::config::standard()).unwrap();
                //println!("PEA SUB {sid} - {}", peasb.aojv.len());

                // --- sub row data 3
                let Ok(buf) = std::fs::read(format!("{DNM}/{sid}-rw3.bin")) else {
                    continue;
                };
                let (v_tras_raw, _): (Vec<PeaAssVar>, usize) =
                    bincode::decode_from_slice(&buf[..], bincode::config::standard()).unwrap();
                if v_tras_raw.is_empty() {
                    println!("    {sid} - NO data ");
                    continue;
                }
                let tras = &v_tras_raw[0];
                let mut sbas = PeaAssVar::from(0u64);
                sbas.arid = aid.to_string();
                sbas.pvid = pid.to_string();
                sbas.sbid = tras.sbid.to_string();
                let note = if subhs.contains(&sbas.sbid) {
                    1f32
                } else {
                    0f32
                };

                let mut m_aoj = HashMap::<String, String>::new();
                for tras in &v_tras_raw {
                    sbas.add(tras);
                    let aoj = tras.aoj.clone();
                    m_aoj.entry(aoj.clone()).or_insert_with(|| aoj.clone());
                }
                /*
                let ar_e = pea.aream.entry(ar).or_insert_with(|| PeaArea {
                    arid: sf.arid.to_string(),
                    ..Default::default()
                });
                        */
                let mut aoj = String::new();
                for (_, v) in &m_aoj {
                    use std::fmt::Write;
                    if !aoj.is_empty() {
                        write!(aoj, ",").unwrap();
                    }
                    write!(aoj, "{}", v).unwrap();
                }
                sbas.aoj = "AOJ".to_string();
                sbas.aoj = aoj;
                sbas.aojv = sb.aojv.clone();
                sbas.copy(tras, VarType::NewCarReg);
                sbas.copy(tras, VarType::Gpp);
                sbas.copy(tras, VarType::MaxPosPowSub);
                sbas.copy(tras, VarType::MaxNegPowSub);
                sbas.copy(tras, VarType::VsppMv);
                sbas.copy(tras, VarType::SppHv);
                sbas.copy(tras, VarType::BigLotMv);
                sbas.copy(tras, VarType::BigLotHv);
                sbas.copy(tras, VarType::SubPowCap);
                sbas.copy(tras, VarType::SolarEnergy);
                let solar = sbas.v[VarType::SolarEnergy as usize].v;
                if solar > 0f32 {
                    println!(">>>>>>>>>>> {sid} solar: {solar} =============");
                }

                // re-calculation of value
                sbas.v[VarType::LvPowSatTr as usize].v =
                    sbas.v[VarType::PkPowTr as usize].v / z2o(sbas.v[VarType::PwCapTr as usize].v);
                sbas.v[VarType::CntLvPowSatTr as usize].v =
                    if sbas.v[VarType::LvPowSatTr as usize].v > 0.8f32 {
                        1f32
                    } else {
                        0f32
                    };
                sbas.v[VarType::ChgStnCap as usize].v = sbas.v[VarType::ChgStnCapTr as usize].v;
                sbas.v[VarType::ChgStnSell as usize].v = sbas.v[VarType::ChgStnSellTr as usize].v;
                sbas.v[VarType::MvPowSatTr as usize].v = sbas.v[VarType::MaxPosPowSub as usize].v
                    / z2o(sbas.v[VarType::SubPowCap as usize].v);
                sbas.v[VarType::MvVspp as usize].v = sbas.v[VarType::VsppMv as usize].v;
                sbas.v[VarType::HvSpp as usize].v = sbas.v[VarType::SppHv as usize].v;
                sbas.v[VarType::SmallSell as usize].v = sbas.v[VarType::SmallSellTr as usize].v;
                sbas.v[VarType::LargeSell as usize].v = sbas.v[VarType::LargeSellTr as usize].v;
                sbas.v[VarType::UnbalPow as usize].v = sbas.v[VarType::UnbalPowTr as usize].v;
                let v = sbas.v[VarType::UnbalPowTr as usize].v
                    / z2o(sbas.v[VarType::PwCapTr as usize].v);
                sbas.v[VarType::CntUnbalPow as usize].v = if v > 0.5f32 { 1f32 } else { 0f32 };
                // end of recalculation

                sbas.v[VarType::TakeNote as usize].v = note;
                sbas_mx.max(&sbas);
                //if let (Some(sbtr), Some(gs)) = (sbtr.get(&sb), sb_inf.get(&sb)) {
                if let Some(sbtr) = sbtr.get(&sbas.sbid) {
                    use sglib03::prc4::ld_ben_bess1;
                    let ben = ld_ben_bess1(&sbas.sbid);
                    let ben8 = ben_bill_accu(sbtr, &ben);
                    let ben9 = ben_cash_flow(sbtr, &ben);
                    let ben10 = ben_dr_save(sbtr, &ben);
                    let mut ben11 = BenProj { proj: emp.clone() };
                    let mut ben12 = BenProj { proj: emp.clone() };
                    let mut ben13 = BenProj { proj: emp.clone() };
                    let mut ben14 = BenProj { proj: emp.clone() };
                    //print!(" {}", sb.sbtp);
                    if ben.mx_pw > 0f32 && ben.grw < 7f32 && ben.be_start <= 3 && ben.trlm > 40f32
                    //&& (sb.conf == "AIS" || sb.conf == "GIS")
                    {
                        let (be_sub_save, be_re_diff, be_svg_save, be_en_added) =
                            ben_amt_proj(&ben);
                        ben11 = be_sub_save;
                        ben12 = be_svg_save;
                        ben13 = be_en_added;
                        ben14 = be_re_diff;
                    }
                    let ben15 = ben_boxline_save(sbtr, &ben);
                    let ben16 = ben_work_save(sbtr, &ben);
                    let ben17 = ben_sell_meter(sbtr, &ben);
                    let ben18 = ben_emeter(sbtr, &ben);
                    let ben19 = ben_mt_read(sbtr, &ben);
                    let ben20 = ben_mt_disconn(sbtr, &ben);
                    let ben21 = ben_tou_sell(sbtr, &ben);
                    let ben22 = ben_tou_read(sbtr, &ben);
                    let ben23 = ben_tou_update(sbtr, &ben);
                    let ben24 = ben_outage_labor(sbtr, &ben);
                    let ben25 = ben_reduce_complain(sbtr, &ben);
                    let ben26 = ben_asset_value(sbtr, &ben);
                    let ben27 = ben_model_entry(sbtr, &ben);
                    let mut ben8v = ben8.proj.iter().map(|(_, b)| *b).collect::<Vec<f32>>();
                    let mut ben9v = ben9.proj.iter().map(|(_, b)| *b).collect::<Vec<f32>>();
                    let mut ben10v = ben10.proj.iter().map(|(_, b)| *b).collect::<Vec<f32>>();
                    let mut ben11v = ben11.proj.iter().map(|(_, b)| *b).collect::<Vec<f32>>();
                    let mut ben12v = ben12.proj.iter().map(|(_, b)| *b).collect::<Vec<f32>>();
                    let mut ben13v = ben13.proj.iter().map(|(_, b)| *b).collect::<Vec<f32>>();
                    let mut ben14v = ben14.proj.iter().map(|(_, b)| *b).collect::<Vec<f32>>();
                    let mut ben15v = ben15.proj.iter().map(|(_, b)| *b).collect::<Vec<f32>>();
                    let mut ben16v = ben16.proj.iter().map(|(_, b)| *b).collect::<Vec<f32>>();
                    let mut ben17v = ben17.proj.iter().map(|(_, b)| *b).collect::<Vec<f32>>();
                    let mut ben18v = ben18.proj.iter().map(|(_, b)| *b).collect::<Vec<f32>>();
                    let mut ben19v = ben19.proj.iter().map(|(_, b)| *b).collect::<Vec<f32>>();
                    let mut ben20v = ben20.proj.iter().map(|(_, b)| *b).collect::<Vec<f32>>();
                    let mut ben21v = ben21.proj.iter().map(|(_, b)| *b).collect::<Vec<f32>>();
                    let mut ben22v = ben22.proj.iter().map(|(_, b)| *b).collect::<Vec<f32>>();
                    let mut ben23v = ben23.proj.iter().map(|(_, b)| *b).collect::<Vec<f32>>();
                    let mut ben24v = ben24.proj.iter().map(|(_, b)| *b).collect::<Vec<f32>>();
                    let mut ben25v = ben25.proj.iter().map(|(_, b)| *b).collect::<Vec<f32>>();
                    let mut ben26v = ben26.proj.iter().map(|(_, b)| *b).collect::<Vec<f32>>();
                    let mut ben27v = ben27.proj.iter().map(|(_, b)| *b).collect::<Vec<f32>>();
                    sbas.vy[VarType::FirBilAccu.tousz()].append(&mut ben8v);
                    sbas.vy[VarType::FirCashFlow.tousz()].append(&mut ben9v);
                    sbas.vy[VarType::FirDRSave.tousz()].append(&mut ben10v);
                    sbas.vy[VarType::FirBatSubSave.tousz()].append(&mut ben11v);
                    sbas.vy[VarType::FirBatSvgSave.tousz()].append(&mut ben12v);
                    sbas.vy[VarType::FirBatEnerSave.tousz()].append(&mut ben13v);
                    sbas.vy[VarType::FirBatPriceDiff.tousz()].append(&mut ben14v);
                    sbas.vy[VarType::FirMetBoxSave.tousz()].append(&mut ben15v);
                    sbas.vy[VarType::FirLaborSave.tousz()].append(&mut ben16v);
                    sbas.vy[VarType::FirMetSell.tousz()].append(&mut ben17v);
                    sbas.vy[VarType::FirEMetSave.tousz()].append(&mut ben18v);
                    sbas.vy[VarType::FirMetReadSave.tousz()].append(&mut ben19v);
                    sbas.vy[VarType::FirMetDisSave.tousz()].append(&mut ben20v);
                    sbas.vy[VarType::FirTouSell.tousz()].append(&mut ben21v);
                    sbas.vy[VarType::FirTouReadSave.tousz()].append(&mut ben22v);
                    sbas.vy[VarType::FirTouUpdateSave.tousz()].append(&mut ben23v);
                    sbas.vy[VarType::FirOutLabSave.tousz()].append(&mut ben24v);
                    sbas.vy[VarType::FirComplainSave.tousz()].append(&mut ben25v);
                    sbas.vy[VarType::FirAssetValue.tousz()].append(&mut ben26v);
                    sbas.vy[VarType::FirDataEntrySave.tousz()].append(&mut ben27v);
                }
                //for tr in v_tras_raw.iter_mut() {
                //sbas.copy(tras, VarType::SolarEnergy);

                // calculation
                //}
                if sbas.v[VarType::TakeNote as usize].v == 1f32 {
                    pvas.add(&sbas);
                }
                pvas.copy(tras, VarType::NewCarReg);
                pvas.copy(tras, VarType::Gpp);

                v_sbas.push(sbas);
                //println!("   {sid} - {}", v_tras.len());
            } // end sub loop

            // re-calculation of value
            pvas.v[VarType::LvPowSatTr as usize].v =
                pvas.v[VarType::PkPowTr as usize].v / z2o(pvas.v[VarType::PwCapTr as usize].v);
            pvas.v[VarType::CntLvPowSatTr as usize].v =
                if pvas.v[VarType::LvPowSatTr as usize].v > 0.8f32 {
                    1f32
                } else {
                    0f32
                };
            pvas.v[VarType::ChgStnCap as usize].v = pvas.v[VarType::ChgStnCapTr as usize].v;
            pvas.v[VarType::ChgStnSell as usize].v = pvas.v[VarType::ChgStnSellTr as usize].v;
            pvas.v[VarType::MvPowSatTr as usize].v = pvas.v[VarType::MaxPosPowSub as usize].v
                / z2o(pvas.v[VarType::SubPowCap as usize].v);
            pvas.v[VarType::MvVspp as usize].v = pvas.v[VarType::VsppMv as usize].v;
            pvas.v[VarType::HvSpp as usize].v = pvas.v[VarType::SppHv as usize].v;
            pvas.v[VarType::SmallSell as usize].v = pvas.v[VarType::SmallSellTr as usize].v;
            pvas.v[VarType::LargeSell as usize].v = pvas.v[VarType::LargeSellTr as usize].v;
            pvas.v[VarType::UnbalPow as usize].v = pvas.v[VarType::UnbalPowTr as usize].v;
            let v =
                pvas.v[VarType::UnbalPowTr as usize].v / z2o(pvas.v[VarType::PwCapTr as usize].v);
            pvas.v[VarType::CntUnbalPow as usize].v = if v > 0.5f32 { 1f32 } else { 0f32 };
            // end of recalculation

            v_pvas.push(pvas);
        } // end provi loop
    } // end area
    let mut uc1_v: Vec<_> = v_sbas
        .iter()
        .enumerate()
        .map(|(i, s)| (s.v[VarType::Uc1Val as usize].v, i))
        .collect();
    uc1_v.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
    for (r, (_, i)) in uc1_v.iter().enumerate() {
        v_sbas[*i].v[VarType::Uc1Rank as usize].v = r as f32;
    }

    let mut uc2_v: Vec<_> = v_sbas
        .iter()
        .enumerate()
        .map(|(i, s)| (s.v[VarType::Uc2Val as usize].v, i))
        .collect();
    uc2_v.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
    for (r, (_, i)) in uc2_v.iter().enumerate() {
        v_sbas[*i].v[VarType::Uc2Rank as usize].v = r as f32;
    }

    let mut uc3_v: Vec<_> = v_sbas
        .iter()
        .enumerate()
        .map(|(i, s)| (s.v[VarType::Uc3Val as usize].v, i))
        .collect();
    uc3_v.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
    for (r, (_, i)) in uc3_v.iter().enumerate() {
        v_sbas[*i].v[VarType::Uc3Rank as usize].v = r as f32;
    }

    // save ev bin
    let bin: Vec<u8> = bincode::encode_to_vec(&v_sbas, bincode::config::standard()).unwrap();
    std::fs::write(format!("{DNM}/000-sbrw.bin"), bin).unwrap();
    write_trn_ass_02(&v_sbas, &format!("{DNM}/000-sbrw0.txt"))?;
    //write_ass_csv_02(&v_sbas, &format!("{DNM}/000-sbrw0.csv"))?;

    println!("SBAS MAX:{:?}", sbas_mx.v);
    let mut v_sbas_no = v_sbas.clone();
    for sub in v_sbas_no.iter_mut() {
        sub.nor(&sbas_mx);
    }
    let bin: Vec<u8> = bincode::encode_to_vec(&v_sbas_no, bincode::config::standard()).unwrap();
    std::fs::write(format!("{DNM}/000-sbno.bin"), bin).unwrap();
    write_trn_ass_02(&v_sbas_no, &format!("{DNM}/000-sbno0.txt"))?;
    //write_ass_csv_02(&v_sbas_no, &format!("{DNM}/000-sbno0.csv"))?;

    let mut ben80 = 0.0;
    for pvas in &v_pvas {
        let ben8n = pvas.vy[VarType::FirBilAccu.tousz()].len();
        let mut ben8a = 0.0;
        for b8 in &pvas.vy[VarType::FirBilAccu.tousz()] {
            ben8a += b8;
        }
        ben80 += ben8a;
        println!("{} - {ben8n} = {ben8a}", pvas.pvid);
    }
    println!("{ben80}");
    let bin: Vec<u8> = bincode::encode_to_vec(&v_pvas, bincode::config::standard()).unwrap();
    std::fs::write(format!("{DNM}/000-pvrw.bin"), bin).unwrap();
    //write_trn_ass_02(&v_pvas, &format!("{DNM}/000-pvrw.txt"))?;
    //write_ass_csv_02(&v_sbas_no, &format!("{DNM}/000-pvrw.csv"))?;
    Ok(())
}

pub fn ben_asset_value(sbtr: &SubCalc, ben: &SubBenInfo) -> BenProj {
    //print!("====  ASSET");
    let m1i = sbtr.mt_1_ph as f64 * M1P_COST as f64;
    let m3i = sbtr.mt_3_ph as f64 * M3P_COST as f64;
    let txp = sbtr.p_tx_cn_m.iter().map(|(_, v)| v).sum::<u32>();
    let txc = sbtr.c_tx_cn_m.iter().map(|(_, v)| v).sum::<u32>();
    let txi = (txp + txc) as f64 * TRX_COST as f64;
    let mut esi = 0f64;
    if ben.mx_pw > 0f32 && ben.grw < 7f32 && ben.be_start <= 3 && ben.trlm > 40f32 {
        esi = ben.bat_cost as f64 * 1_000_000_f64;
    }
    let ass = (m1i + m3i + txi + esi) * ASSET_WORTH_RATIO as f64;
    //print!("  m1:{m1i} m3:{m3i} t:{txi} b:{esi} = as:{ass}\n");
    let mut proj = Vec::<(u32, f32)>::new();
    for y in 0..11 {
        proj.push((y + 2028, 0f32));
    }
    proj.push((11 + 2028, ass as f32));
    //println!();
    BenProj { proj }
}

pub fn ben_model_entry(sbtr: &SubCalc, ben: &SubBenInfo) -> BenProj {
    //print!("====  MODEL ENTRY");
    let txp = sbtr.p_tx_cn_m.iter().map(|(_, v)| v).sum::<u32>();
    let txc = sbtr.c_tx_cn_m.iter().map(|(_, v)| v).sum::<u32>();
    let mut cnt = (txp + txc + sbtr.mt_1_ph as u32 + sbtr.mt_3_ph as u32) as f64;
    if ben.mx_pw > 0f32 && ben.grw < 7f32 && ben.be_start <= 3 && ben.trlm > 40f32 {
        cnt += 1.0;
    }
    let ent_cn = cnt * MODEL_ENTRY_RATIO as f64;
    let ent_ex = ent_cn * MODEL_ENTRY_COST as f64;

    //print!("  cn:{ent_cn} ex:{ent_ex} \n");
    let mut proj = Vec::<(u32, f32)>::new();
    for y in 0..12 {
        let be = ent_ex;
        let be = be * Pow::pow(1f64 + CALL_CENTER_COST_UP as f64, y as f64);
        //print!(" {} - {be}", y + 2028);
        proj.push((y + 2028, be as f32));
    }
    //println!();
    BenProj { proj }
}

pub fn c01_chk_03_2() -> Result<(), Box<dyn Error>> {
    let buf = std::fs::read(format!("{DNM}/000_pea.bin")).unwrap();
    let (pea, _): (Pea, usize) =
        bincode::decode_from_slice(&buf[..], bincode::config::standard()).unwrap();
    let mut aids: Vec<_> = pea.aream.keys().collect();
    aids.sort();
    //println!("..1");
    let subhs = p01_chk();
    //println!("..1.1");
    //let sbtr = ld_sb_tr0();
    let sbtr = ld_sub_calc();
    //println!("..1.2");
    //println!("sbtr: {}", sbtr.len());
    let mut emp = Vec::<(u32, f32)>::new();
    for y in OP_YEAR_START..=OP_YEAR_END {
        emp.push((y, 0f32));
    }
    //
    //let mut pvcn = 0;
    //println!("..2");
    let mut v_pvas = Vec::<PeaAssVar>::new();
    let mut v_sbas = Vec::<PeaAssVar>::new();
    //let mut sbas_mx = PeaAssVar::default();
    let mut sbas_mx = PeaAssVar::from(0u64);
    for aid in aids {
        //println!("..3");
        let Some(ar) = pea.aream.get(aid) else {
            continue;
        };
        //println!("..4");
        let mut pids: Vec<_> = ar.provm.keys().collect();
        pids.sort();
        for pid in pids {
            //println!("..5");
            let Some(prov) = ar.provm.get(pid) else {
                continue;
            };
            //println!("..6");
            let mut pvas = PeaAssVar::from(0u64);
            pvas.arid = aid.to_string();
            pvas.pvid = pid.to_string();
            println!("  pv:{pid}");
            let mut sids: Vec<_> = prov.subm.keys().collect();
            sids.sort();
            for sid in sids {
                let Some(_sb) = prov.subm.get(sid) else {
                    continue;
                };
                let Ok(buf) = std::fs::read(format!("{DNM}/{sid}-rw3.bin")) else {
                    continue;
                };
                let (v_tras_raw, _): (Vec<PeaAssVar>, usize) =
                    bincode::decode_from_slice(&buf[..], bincode::config::standard()).unwrap();
                if v_tras_raw.is_empty() {
                    println!("    {sid} - NO data ");
                    continue;
                }
                let tras = &v_tras_raw[0];
                let mut sbas = PeaAssVar::from(0u64);
                sbas.arid = aid.to_string();
                sbas.pvid = pid.to_string();
                sbas.sbid = tras.sbid.to_string();
                let note = if subhs.contains(&sbas.sbid) {
                    1f32
                } else {
                    0f32
                };

                let mut m_aoj = HashMap::<String, String>::new();
                for tras in &v_tras_raw {
                    sbas.add(tras);
                    let aoj = tras.aoj.clone();
                    m_aoj.entry(aoj.clone()).or_insert_with(|| aoj.clone());
                }
                /*
                let ar_e = pea.aream.entry(ar).or_insert_with(|| PeaArea {
                    arid: sf.arid.to_string(),
                    ..Default::default()
                });
                        */
                let mut aoj = String::new();
                for (_, v) in &m_aoj {
                    use std::fmt::Write;
                    if !aoj.is_empty() {
                        write!(aoj, ",").unwrap();
                    }
                    write!(aoj, "{}", v).unwrap();
                }
                sbas.aoj = "AOJ".to_string();
                sbas.aoj = aoj;
                sbas.copy(tras, VarType::NewCarReg);
                sbas.copy(tras, VarType::Gpp);
                sbas.copy(tras, VarType::MaxPosPowSub);
                sbas.copy(tras, VarType::MaxNegPowSub);
                sbas.copy(tras, VarType::VsppMv);
                sbas.copy(tras, VarType::SppHv);
                sbas.copy(tras, VarType::BigLotMv);
                sbas.copy(tras, VarType::BigLotHv);
                sbas.copy(tras, VarType::SubPowCap);
                sbas.copy(tras, VarType::SolarEnergy);
                let solar = sbas.v[VarType::SolarEnergy as usize].v;
                if solar > 0f32 {
                    println!(">>>>>>>>>>> {sid} solar: {solar} =============");
                }

                // re-calculation of value
                sbas.v[VarType::LvPowSatTr as usize].v =
                    sbas.v[VarType::PkPowTr as usize].v / z2o(sbas.v[VarType::PwCapTr as usize].v);
                sbas.v[VarType::CntLvPowSatTr as usize].v =
                    if sbas.v[VarType::LvPowSatTr as usize].v > 0.8f32 {
                        1f32
                    } else {
                        0f32
                    };
                sbas.v[VarType::ChgStnCap as usize].v = sbas.v[VarType::ChgStnCapTr as usize].v;
                sbas.v[VarType::ChgStnSell as usize].v = sbas.v[VarType::ChgStnSellTr as usize].v;
                sbas.v[VarType::MvPowSatTr as usize].v = sbas.v[VarType::MaxPosPowSub as usize].v
                    / z2o(sbas.v[VarType::SubPowCap as usize].v);
                sbas.v[VarType::MvVspp as usize].v = sbas.v[VarType::VsppMv as usize].v;
                sbas.v[VarType::HvSpp as usize].v = sbas.v[VarType::SppHv as usize].v;
                sbas.v[VarType::SmallSell as usize].v = sbas.v[VarType::SmallSellTr as usize].v;
                sbas.v[VarType::LargeSell as usize].v = sbas.v[VarType::LargeSellTr as usize].v;
                sbas.v[VarType::UnbalPow as usize].v = sbas.v[VarType::UnbalPowTr as usize].v;
                let v = sbas.v[VarType::UnbalPowTr as usize].v
                    / z2o(sbas.v[VarType::PwCapTr as usize].v);
                sbas.v[VarType::CntUnbalPow as usize].v = if v > 0.5f32 { 1f32 } else { 0f32 };
                // end of recalculation

                sbas.v[VarType::TakeNote as usize].v = note;
                sbas_mx.max(&sbas);
                //if let (Some(sbtr), Some(gs)) = (sbtr.get(&sb), sb_inf.get(&sb)) {
                if let Some(sbtr) = sbtr.get(&sbas.sbid) {
                    use sglib03::prc4::ld_ben_bess1;
                    let ben = ld_ben_bess1(&sbas.sbid);
                    let ben8 = ben_bill_accu(sbtr, &ben);
                    let ben9 = ben_cash_flow(sbtr, &ben);
                    let ben10 = ben_dr_save(sbtr, &ben);
                    let mut ben11 = BenProj { proj: emp.clone() };
                    let mut ben12 = BenProj { proj: emp.clone() };
                    let mut ben13 = BenProj { proj: emp.clone() };
                    let mut ben14 = BenProj { proj: emp.clone() };
                    //print!(" {}", sb.sbtp);
                    if ben.mx_pw > 0f32 && ben.grw < 7f32 && ben.be_start <= 3 && ben.trlm > 40f32
                    //&& (sb.conf == "AIS" || sb.conf == "GIS")
                    {
                        let (be_sub_save, be_re_diff, be_svg_save, be_en_added) =
                            ben_amt_proj(&ben);
                        ben11 = be_sub_save;
                        ben12 = be_svg_save;
                        ben13 = be_en_added;
                        ben14 = be_re_diff;
                    }
                    let ben15 = ben_boxline_save(sbtr, &ben);
                    let ben16 = ben_work_save(sbtr, &ben);
                    let ben17 = ben_sell_meter(sbtr, &ben);
                    let ben18 = ben_emeter(sbtr, &ben);
                    let ben19 = ben_mt_read(sbtr, &ben);
                    let ben20 = ben_mt_disconn(sbtr, &ben);
                    let ben21 = ben_tou_sell(sbtr, &ben);
                    let ben22 = ben_tou_read(sbtr, &ben);
                    let ben23 = ben_tou_update(sbtr, &ben);
                    let ben24 = ben_outage_labor(sbtr, &ben);
                    let ben25 = ben_reduce_complain(sbtr, &ben);
                    let ben26 = ben_asset_value(sbtr, &ben);
                    let ben27 = ben_model_entry(sbtr, &ben);
                    let mut ben8v = ben8.proj.iter().map(|(_, b)| *b).collect::<Vec<f32>>();
                    let mut ben9v = ben9.proj.iter().map(|(_, b)| *b).collect::<Vec<f32>>();
                    let mut ben10v = ben10.proj.iter().map(|(_, b)| *b).collect::<Vec<f32>>();
                    let mut ben11v = ben11.proj.iter().map(|(_, b)| *b).collect::<Vec<f32>>();
                    let mut ben12v = ben12.proj.iter().map(|(_, b)| *b).collect::<Vec<f32>>();
                    let mut ben13v = ben13.proj.iter().map(|(_, b)| *b).collect::<Vec<f32>>();
                    let mut ben14v = ben14.proj.iter().map(|(_, b)| *b).collect::<Vec<f32>>();
                    let mut ben15v = ben15.proj.iter().map(|(_, b)| *b).collect::<Vec<f32>>();
                    let mut ben16v = ben16.proj.iter().map(|(_, b)| *b).collect::<Vec<f32>>();
                    let mut ben17v = ben17.proj.iter().map(|(_, b)| *b).collect::<Vec<f32>>();
                    let mut ben18v = ben18.proj.iter().map(|(_, b)| *b).collect::<Vec<f32>>();
                    let mut ben19v = ben19.proj.iter().map(|(_, b)| *b).collect::<Vec<f32>>();
                    let mut ben20v = ben20.proj.iter().map(|(_, b)| *b).collect::<Vec<f32>>();
                    let mut ben21v = ben21.proj.iter().map(|(_, b)| *b).collect::<Vec<f32>>();
                    let mut ben22v = ben22.proj.iter().map(|(_, b)| *b).collect::<Vec<f32>>();
                    let mut ben23v = ben23.proj.iter().map(|(_, b)| *b).collect::<Vec<f32>>();
                    let mut ben24v = ben24.proj.iter().map(|(_, b)| *b).collect::<Vec<f32>>();
                    let mut ben25v = ben25.proj.iter().map(|(_, b)| *b).collect::<Vec<f32>>();
                    let mut ben26v = ben26.proj.iter().map(|(_, b)| *b).collect::<Vec<f32>>();
                    let mut ben27v = ben27.proj.iter().map(|(_, b)| *b).collect::<Vec<f32>>();
                    sbas.vy[VarType::FirBilAccu.tousz()].append(&mut ben8v);
                    sbas.vy[VarType::FirCashFlow.tousz()].append(&mut ben9v);
                    sbas.vy[VarType::FirDRSave.tousz()].append(&mut ben10v);
                    sbas.vy[VarType::FirBatSubSave.tousz()].append(&mut ben11v);
                    sbas.vy[VarType::FirBatSvgSave.tousz()].append(&mut ben12v);
                    sbas.vy[VarType::FirBatEnerSave.tousz()].append(&mut ben13v);
                    sbas.vy[VarType::FirBatPriceDiff.tousz()].append(&mut ben14v);
                    sbas.vy[VarType::FirMetBoxSave.tousz()].append(&mut ben15v);
                    sbas.vy[VarType::FirLaborSave.tousz()].append(&mut ben16v);
                    sbas.vy[VarType::FirMetSell.tousz()].append(&mut ben17v);
                    sbas.vy[VarType::FirEMetSave.tousz()].append(&mut ben18v);
                    sbas.vy[VarType::FirMetReadSave.tousz()].append(&mut ben19v);
                    sbas.vy[VarType::FirMetDisSave.tousz()].append(&mut ben20v);
                    sbas.vy[VarType::FirTouSell.tousz()].append(&mut ben21v);
                    sbas.vy[VarType::FirTouReadSave.tousz()].append(&mut ben22v);
                    sbas.vy[VarType::FirTouUpdateSave.tousz()].append(&mut ben23v);
                    sbas.vy[VarType::FirOutLabSave.tousz()].append(&mut ben24v);
                    sbas.vy[VarType::FirComplainSave.tousz()].append(&mut ben25v);
                    sbas.vy[VarType::FirAssetValue.tousz()].append(&mut ben26v);
                    sbas.vy[VarType::FirDataEntrySave.tousz()].append(&mut ben27v);
                }
                //for tr in v_tras_raw.iter_mut() {
                //sbas.copy(tras, VarType::SolarEnergy);

                // calculation
                //}
                if sbas.v[VarType::TakeNote as usize].v == 1f32 {
                    pvas.add(&sbas);
                }
                pvas.copy(tras, VarType::NewCarReg);
                pvas.copy(tras, VarType::Gpp);

                v_sbas.push(sbas);
                //println!("   {sid} - {}", v_tras.len());
            } // end sub loop

            // re-calculation of value
            pvas.v[VarType::LvPowSatTr as usize].v =
                pvas.v[VarType::PkPowTr as usize].v / z2o(pvas.v[VarType::PwCapTr as usize].v);
            pvas.v[VarType::CntLvPowSatTr as usize].v =
                if pvas.v[VarType::LvPowSatTr as usize].v > 0.8f32 {
                    1f32
                } else {
                    0f32
                };
            pvas.v[VarType::ChgStnCap as usize].v = pvas.v[VarType::ChgStnCapTr as usize].v;
            pvas.v[VarType::ChgStnSell as usize].v = pvas.v[VarType::ChgStnSellTr as usize].v;
            pvas.v[VarType::MvPowSatTr as usize].v = pvas.v[VarType::MaxPosPowSub as usize].v
                / z2o(pvas.v[VarType::SubPowCap as usize].v);
            pvas.v[VarType::MvVspp as usize].v = pvas.v[VarType::VsppMv as usize].v;
            pvas.v[VarType::HvSpp as usize].v = pvas.v[VarType::SppHv as usize].v;
            pvas.v[VarType::SmallSell as usize].v = pvas.v[VarType::SmallSellTr as usize].v;
            pvas.v[VarType::LargeSell as usize].v = pvas.v[VarType::LargeSellTr as usize].v;
            pvas.v[VarType::UnbalPow as usize].v = pvas.v[VarType::UnbalPowTr as usize].v;
            let v =
                pvas.v[VarType::UnbalPowTr as usize].v / z2o(pvas.v[VarType::PwCapTr as usize].v);
            pvas.v[VarType::CntUnbalPow as usize].v = if v > 0.5f32 { 1f32 } else { 0f32 };
            // end of recalculation

            v_pvas.push(pvas);
        } // end provi loop
    } // end area
    let mut uc1_v: Vec<_> = v_sbas
        .iter()
        .enumerate()
        .map(|(i, s)| (s.v[VarType::Uc1Val as usize].v, i + 1))
        .collect();
    uc1_v.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
    for (r, (_, i)) in uc1_v.iter().enumerate() {
        v_sbas[*i].v[VarType::Uc1Rank as usize].v = r as f32;
    }

    let mut uc2_v: Vec<_> = v_sbas
        .iter()
        .enumerate()
        .map(|(i, s)| (s.v[VarType::Uc2Val as usize].v, i + 1))
        .collect();
    uc2_v.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
    for (r, (_, i)) in uc2_v.iter().enumerate() {
        v_sbas[*i].v[VarType::Uc2Rank as usize].v = r as f32;
    }

    let mut uc3_v: Vec<_> = v_sbas
        .iter()
        .enumerate()
        .map(|(i, s)| (s.v[VarType::Uc3Val as usize].v, i + 1))
        .collect();
    uc3_v.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
    for (r, (_, i)) in uc3_v.iter().enumerate() {
        v_sbas[*i].v[VarType::Uc3Rank as usize].v = r as f32;
    }

    // save ev bin
    let bin: Vec<u8> = bincode::encode_to_vec(&v_sbas, bincode::config::standard()).unwrap();
    std::fs::write(format!("{DNM}/000-sbrw.bin"), bin).unwrap();
    write_trn_ass_02(&v_sbas, &format!("{DNM}/000-sbrw0.txt"))?;
    //write_ass_csv_02(&v_sbas, &format!("{DNM}/000-sbrw0.csv"))?;

    println!("SBAS MAX:{:?}", sbas_mx.v);
    let mut v_sbas_no = v_sbas.clone();
    for sub in v_sbas_no.iter_mut() {
        sub.nor(&sbas_mx);
    }
    let bin: Vec<u8> = bincode::encode_to_vec(&v_sbas_no, bincode::config::standard()).unwrap();
    std::fs::write(format!("{DNM}/000-sbno.bin"), bin).unwrap();
    write_trn_ass_02(&v_sbas_no, &format!("{DNM}/000-sbno0.txt"))?;
    //write_ass_csv_02(&v_sbas_no, &format!("{DNM}/000-sbno0.csv"))?;

    let mut ben80 = 0.0;
    for pvas in &v_pvas {
        let ben8n = pvas.vy[VarType::FirBilAccu.tousz()].len();
        let mut ben8a = 0.0;
        for b8 in &pvas.vy[VarType::FirBilAccu.tousz()] {
            ben8a += b8;
        }
        ben80 += ben8a;
        println!("{} - {ben8n} = {ben8a}", pvas.pvid);
    }
    println!("{ben80}");
    let bin: Vec<u8> = bincode::encode_to_vec(&v_pvas, bincode::config::standard()).unwrap();
    std::fs::write(format!("{DNM}/000-pvrw.bin"), bin).unwrap();
    //write_trn_ass_02(&v_pvas, &format!("{DNM}/000-pvrw.txt"))?;
    //write_ass_csv_02(&v_sbas_no, &format!("{DNM}/000-pvrw.csv"))?;
    Ok(())
}
