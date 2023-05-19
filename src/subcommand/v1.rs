use {
  super::*,
};

//--------v1 response-------------

#[derive(Serialize, Deserialize)]
pub struct ExportInscriptions<T> {
  pub data: Vec<T>,
  pub next: Option<u64>,
  pub prev: Option<u64>,
}


#[derive(Serialize, Deserialize)]
pub(crate) struct ExportInscription {
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
  pub(crate) address: String,

  pub(crate) offset: u64,
  pub(crate) s_output: String,
  pub(crate) location: String,
  pub(crate) genesis_transaction: String,

}


#[derive(Serialize, Deserialize)]
pub(crate) struct CheckInscriptionId {
  pub(crate) chain: Chain,
  pub(crate) inscription: Option<InscriptionId>,
  pub(crate) txid: Txid,
  pub(crate) number: u64,
  pub(crate) height: u64,
  // pub(crate) inputs: Vec<ExportUTXO>,
}


#[derive(Serialize, Deserialize)]
pub(crate) struct ExportTransaction {
  // pub(crate) blockhash: Option<BlockHash>,
  pub(crate) chain: Chain,
  pub(crate) inscription: Option<InscriptionId>,
  pub(crate) transaction: Transaction,
  pub(crate) txid: Txid,

  pub(crate) inputs: Vec<ExportUTXO>,
  pub(crate) outputs: Vec<ExportUTXO>,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct ExportUTXO {
  pub outpoint: String,
  pub value: String,
  pub script_pubkey: String,
  pub address: String,
}


#[derive(Serialize, Deserialize)]
pub(crate) struct ExportOutput {
  pub(crate) outpoint: OutPoint,
  //pub(crate) list: Option<List>,
  pub(crate) chain: Chain,
  pub(crate) output: TxOut,
  pub(crate) address: String,
  pub(crate) inscriptions: Vec<InscriptionId>,
}




