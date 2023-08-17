use super::super::model::tx::Txout;

#[derive(Debug)]
struct TxoutBuilder {
   neutrinos: u64
}

impl TxoutBuilder {
   fn new() -> TxoutBuilder {
      TxoutBuilder { 
         neutrinos: 0,
      }
   }
}

#[cfg(test)]
mod tests {

   #[test]
   fn test_TxoutBuilder() {

   }
}