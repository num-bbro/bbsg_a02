use sglib04::prc41::SubCalc;
use sglib04::web1::COMM_COST;
use sglib04::web1::ESS_COST;
use sglib04::web1::ESS_OP_COST;
use sglib04::web1::M1P_COST;
use sglib04::web1::M1P_IMP_COST;
use sglib04::web1::M1P_OP_COST;
use sglib04::web1::M3P_COST;
use sglib04::web1::M3P_IMP_COST;
use sglib04::web1::M3P_OP_COST;
use sglib04::web1::PLATFORM_COST;
use sglib04::web1::PLATFORM_OP_COST;
use sglib04::web1::TRX_COST;
use sglib04::web1::TRX_IMP_COST;
use sglib04::web1::TRX_OP_COST;

pub fn cst_m1p_ins(sbtr: &SubCalc) -> Vec<f32> {
    let cst = M1P_COST * sbtr.mt_1_ph as f32 / 3.0;
    let mut csts = Vec::<f32>::new();
    for _i in 0..3 {
        csts.push(cst);
    }
    csts
}

pub fn cst_m3p_ins(sbtr: &SubCalc) -> Vec<f32> {
    let cst = M3P_COST * sbtr.mt_3_ph as f32 / 3.0;
    let mut csts = Vec::<f32>::new();
    for _i in 0..3 {
        csts.push(cst);
    }
    csts
}

pub fn cst_tr_ins(sbtr: &SubCalc) -> Vec<f32> {
    let trp: f32 = sbtr.p_tx_cn_m.values().map(|v| *v as f32).sum();
    let trc: f32 = sbtr.c_tx_cn_m.values().map(|v| *v as f32).sum();
    let cst = TRX_COST * (trp + trc) / 3.0;
    let mut csts = Vec::<f32>::new();
    for _i in 0..3 {
        csts.push(cst);
    }
    csts
}

pub fn cst_bes_ins(_sbtr: &SubCalc, bescap: f32) -> Vec<f32> {
    let cst = ESS_COST * bescap / 3.0;
    let mut csts = Vec::<f32>::new();
    for _i in 0..3 {
        csts.push(cst);
    }
    csts
}

pub fn cst_plfm_ins(sbtr: &SubCalc, bescap: f32) -> Vec<f32> {
    let trp: f32 = sbtr.p_tx_cn_m.values().map(|v| *v as f32).sum();
    let trc: f32 = sbtr.c_tx_cn_m.values().map(|v| *v as f32).sum();
    let cnt = sbtr.mt_1_ph as f32 + sbtr.mt_3_ph as f32 + trp + trc;
    let cnt = if bescap > 0f32 { cnt + 1.0 } else { cnt };
    let cst = PLATFORM_COST * cnt / 3.0;
    let mut csts = Vec::<f32>::new();
    for _i in 0..3 {
        csts.push(cst);
    }
    csts
}
pub fn cst_comm_ins(_sbtr: &SubCalc, _bescap: f32) -> Vec<f32> {
    vec![0.0]
}

pub fn cst_m1p_imp(sbtr: &SubCalc) -> Vec<f32> {
    let cst = M1P_IMP_COST * sbtr.mt_1_ph as f32 / 3.0;
    let mut csts = Vec::<f32>::new();
    for _i in 0..3 {
        csts.push(cst);
    }
    csts
}

pub fn cst_m3p_imp(sbtr: &SubCalc) -> Vec<f32> {
    let cst = M3P_IMP_COST * sbtr.mt_3_ph as f32 / 3.0;
    let mut csts = Vec::<f32>::new();
    for _i in 0..3 {
        csts.push(cst);
    }
    csts
}

pub fn cst_tr_imp(sbtr: &SubCalc) -> Vec<f32> {
    let trp: f32 = sbtr.p_tx_cn_m.values().map(|v| *v as f32).sum();
    let trc: f32 = sbtr.c_tx_cn_m.values().map(|v| *v as f32).sum();
    let cst = TRX_IMP_COST * (trp + trc) / 3.0;
    let mut csts = Vec::<f32>::new();
    for _i in 0..3 {
        csts.push(cst);
    }
    csts
}

pub fn cst_bes_imp(_sbtr: &SubCalc, _bescap: f32) -> Vec<f32> {
    vec![0.0]
}
pub fn cst_plfm_imp(_sbtr: &SubCalc, _bescap: f32) -> Vec<f32> {
    vec![0.0]
}
pub fn cst_comm_imp(_sbtr: &SubCalc, _bescap: f32) -> Vec<f32> {
    vec![0.0]
}

pub fn cst_m1p_op(sbtr: &SubCalc) -> Vec<f32> {
    let cst = M1P_OP_COST * sbtr.mt_1_ph as f32;
    let mut csts = vec![0.0, 0.0, 0.0];
    for _i in 0..12 {
        csts.push(cst);
    }
    csts
}

pub fn cst_m3p_op(sbtr: &SubCalc) -> Vec<f32> {
    let cst = M3P_OP_COST * sbtr.mt_3_ph as f32;
    let mut csts = vec![0.0, 0.0, 0.0];
    for _i in 0..12 {
        csts.push(cst);
    }
    csts
}

pub fn cst_tr_op(sbtr: &SubCalc) -> Vec<f32> {
    let trp: f32 = sbtr.p_tx_cn_m.values().map(|v| *v as f32).sum();
    let trc: f32 = sbtr.c_tx_cn_m.values().map(|v| *v as f32).sum();
    let cst = TRX_OP_COST * (trp + trc);
    let mut csts = vec![0.0, 0.0, 0.0];
    for _i in 0..12 {
        csts.push(cst);
    }
    csts
}

pub fn cst_bes_op(_sbtr: &SubCalc, bescap: f32) -> Vec<f32> {
    let cst = bescap * ESS_OP_COST / 3.0;
    let mut csts = vec![0.0, 0.0, 0.0];
    for _i in 0..12 {
        csts.push(cst);
    }
    csts
}

pub fn cst_plfm_op(sbtr: &SubCalc, bescap: f32) -> Vec<f32> {
    let trp: f32 = sbtr.p_tx_cn_m.values().map(|v| *v as f32).sum();
    let trc: f32 = sbtr.c_tx_cn_m.values().map(|v| *v as f32).sum();
    let cnt = sbtr.mt_1_ph as f32 + sbtr.mt_3_ph as f32 + trp + trc;
    let cnt = if bescap > 0f32 { cnt + 1.0 } else { cnt };
    let cst = PLATFORM_OP_COST * cnt;
    let mut csts = vec![0.0, 0.0, 0.0];
    for _i in 0..12 {
        csts.push(cst);
    }
    csts
}
pub fn cst_comm_op(sbtr: &SubCalc, bescap: f32) -> Vec<f32> {
    let trp: f32 = sbtr.p_tx_cn_m.values().map(|v| *v as f32).sum();
    let trc: f32 = sbtr.c_tx_cn_m.values().map(|v| *v as f32).sum();
    let cnt = sbtr.mt_1_ph as f32 + sbtr.mt_3_ph as f32 + trp + trc;
    let cnt = if bescap > 0f32 { cnt + 1.0 } else { cnt };
    let cst = COMM_COST * cnt;
    let mut csts = vec![0.0, 0.0, 0.0];
    for _i in 0..12 {
        csts.push(cst);
    }
    csts
}
