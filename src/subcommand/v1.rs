use {
  super::*,
};

//--------v1 response-------------

#[derive(Serialize, Deserialize)]
pub struct PageData<T> {
  pub data: Vec<T>,
  pub next: Option<u64>,
  pub prev: Option<u64>,
}


#[derive(Serialize, Deserialize)]
pub(crate) struct InscriptionDetail {
  pub(crate) chain: Chain,
  pub(crate) genesis_fee: u64,
  pub(crate) genesis_height: u64,
  pub(crate) inscription: Inscription,
  pub(crate) inscription_id: InscriptionId,
  pub(crate) next: Option<InscriptionId>,
  pub(crate) number: u64,
  pub(crate) output: TxOut,
  pub(crate) previous: Option<InscriptionId>,
  pub(crate) sat: Option<Sat>,
  pub(crate) satpoint: SatPoint,
  pub(crate) timestamp: u32,
}
