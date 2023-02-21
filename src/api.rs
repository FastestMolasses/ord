use {
  super::{Index, Sat},
  bitcoincore_rpc::Auth,
  once_cell::sync::OnceCell,
  pyo3::{pyfunction, PyResult},
  std::str::FromStr,
  bitcoin::{BlockHash},
};

static DATABASE: OnceCell<Index> = OnceCell::new();

#[pyfunction]
pub(crate) fn init_db(rpc_url: String, username: String, password: String) {
  DATABASE
    .set(
      Index::init_db(
        rpc_url,
        Auth::UserPass(username, password),
        &std::env::current_dir().unwrap().join("index.db"),
        crate::chain::Chain::Mainnet,
      )
      .unwrap(),
    )
    .or(Err("Failed to initialize database"))
    .unwrap();
}

#[pyfunction]
pub(crate) fn index_blockchain() {
  DATABASE.get().unwrap().update().unwrap();
}

#[pyfunction]
pub(crate) fn get_block_count() -> PyResult<u64> {
  return Ok(DATABASE.get().unwrap().block_count().unwrap())
}

#[pyfunction]
pub(crate) fn get_block_from_height(height: u64) -> PyResult<String> {
  let block = DATABASE.get().unwrap().get_block_by_height(height).unwrap();
  return Ok(format!("{:?}", block));
}

#[pyfunction]
pub(crate) fn get_block_from_hash(hash: &str) {
  let index = DATABASE.get().unwrap();
  let block = index
        .get_block_by_hash(BlockHash::from_str(hash).unwrap()).unwrap();

  println!("{:?}", block);
}

// #[pyfunction]
// pub(crate) fn get_inscription(id: String) {
//   let inscription_id = InscriptionId::from_str(&id).unwrap();

//   let entry = DATABASE
//     .get()
//     .unwrap()
//     .get_inscription_entry(inscription_id)
//     .unwrap()
//     .unwrap();
//   let inscription = DATABASE
//     .get()
//     .unwrap()
//     .get_inscription_by_id(inscription_id)
//     .unwrap()
//     .unwrap();
//   let satpoint = DATABASE
//     .get()
//     .unwrap()
//     .get_inscription_satpoint_by_id(inscription_id)
//     .unwrap()
//     .unwrap();
//   let output = DATABASE
//     .get()
//     .unwrap()
//     .get_transaction(satpoint.outpoint.txid)
//     .unwrap()
//     .into_iter()
//     .nth(satpoint.outpoint.vout.try_into().unwrap());

//   let previous = if let Some(previous) = entry.number.checked_sub(1) {
//     Some(
//       DATABASE
//         .get()
//         .unwrap()
//         .get_inscription_id_by_inscription_number(previous)
//         .unwrap(),
//     )
//   } else {
//     None
//   };

//   let next = DATABASE
//     .get()
//     .unwrap()
//     .get_inscription_id_by_inscription_number(entry.number + 1);

//   // Create a hashmap to return
//   let mut response = HashMap::new();
//   response.insert(
//     "entry".to_string(),
//     format!("{:?}", entry),
//   );
//   response.insert(
//     "".to_string(),
//     format!("{:?}", block),
//   );
//   response.insert(
//     "".to_string(),
//     format!("{:?}", block),
//   );
//   response.insert(
//     "".to_string(),
//     format!("{:?}", block),
//   );
// }

#[pyfunction]
pub(crate) fn get_latest_inscriptions(amount: usize, from: Option<u64>) -> PyResult<Vec<String>> {
  let (inscriptions, _, _) = DATABASE
    .get()
    .unwrap()
    .get_latest_inscriptions_with_prev_and_next(amount, from)
    .unwrap();

  // Create a vector of the inscriptions as strings
  let inscriptions: Vec<String> = inscriptions
    .iter()
    .map(|inscription| inscription.to_string())
    .collect();
  return Ok(inscriptions);
}

#[pyfunction]
pub(crate) fn get_sat(sat: u64) -> PyResult<String> {
  let index = DATABASE.get().unwrap();
  let sat = Sat(sat);
  let satpoint = index.rare_sat_satpoint(sat).unwrap();
  let blocktime = index.blocktime(sat.height()).unwrap();
  let inscription = index.get_inscription_id_by_sat(sat).unwrap();
  return Ok(inscription.unwrap().to_string());
}

// #[pyfunction]
// pub(crate) fn db_info() -> PyResult<Value> {
//   let index = DATABASE.get().unwrap();
//   return Ok(json!(index.info().unwrap()));
// }
