//Trabalho individual

use std::io;
use std::fs::File;
use std::io::BufRead;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
struct Reserva{
  num_reserva: i32,
  nome_hotel: String,
  num_quarto: i32,
  data_checkin: String,
  data_checkout: String,
}
impl Reserva {
fn new(num_reserva: i32, nome_hotel: String, num_quarto: i32, data_checkin: String, data_checkout: String) -> Self {
    Self {
      num_reserva,
      nome_hotel,
      num_quarto,
      data_checkin,
      data_checkout,
    }
  }
}

#[derive(Debug)]
struct SistemaDeReserva {
    reservas: HashMap<i32, Vec<Reserva>>,
}
impl SistemaDeReserva{
fn new() -> Self {
    Self{
      reservas: HashMap::new(),
    }
  }
//adição de reservas dos arquivos (não aparece mensagens de confirmação)
fn adicionar_reserva_arquivo(&mut self, chave: i32, reserva: Reserva) {
  let lista_reservas = self.reservas.entry(chave).or_insert(Vec::new());
  lista_reservas.push(reserva.clone());
  }
//adição de reserva (esse será o que o usuário irá interagir)(aparece mensagens de confirmação)
fn adicionar_reserva(&mut self, chave: i32, reserva: Reserva) {
  let lista_reservas = self.reservas.entry(chave).or_insert(Vec::new());
  if lista_reservas.len() >= 1 {
    loop {
    let mut input = String::new();
    println!("\nJá existe uma (ou mais) chave associada ao número de reserva: {}. Deseja adicionar a reserva à lista desta mesma chave? \n1 - Sim \n2 - Não", reserva.num_reserva);
    io::stdin().read_line(&mut input).unwrap();
    let opcao: i32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Número digitado inválido!");
            continue;
        }
    };
    if opcao == 1 {
      lista_reservas.push(reserva.clone());
      println!("\nReserva adicionada à lista de espera com sucesso! \nNúmero da reserva: {}", reserva.num_reserva);
      break;
    } else if opcao == 2 {
      println!("\nReserva não adicionada.");
      break;
    } else {
      println!("\nOpção inválida!");
      println!("Digite um número conforme as instruções de resposta.");
      continue;
    }
  }
    } else {
       lista_reservas.push(reserva.clone());
        println!("\nReserva adicionada com sucesso! \nNúmero da reserva: {}", reserva.num_reserva);
    }
  }
//pesquisa de reservas
fn detalhes_reserva(&self, chave: i32){
    if let Some(lista_reservas) = self.reservas.get(&chave) {
      for reserva in lista_reservas {
          println!("\nChave: {}", chave);
          println!("Número da reserva: {}", reserva.num_reserva);
          println!("Nome do hotel: {}", reserva.nome_hotel);
          println!("Número do quarto: {}", reserva.num_quarto);
          println!("Data de check-in: {}", reserva.data_checkin);
          println!("Data de check-out: {}", reserva.data_checkout);
        }
    } else {
          println!("\nReserva não encontrada!");
    }
  }
//remoção de reservas
fn remover_reserva(&mut self, chave: i32, reserva: &Reserva) {
      if let Some(lista_reservas) = self.reservas.get_mut(&chave) {
          if let Some(pos) = lista_reservas.iter().position(|x| *x == *reserva) {
              lista_reservas.remove(pos);
              println!("\nReserva removida com sucesso! \nNúmero da reserva: {}", reserva.num_reserva);
              if lista_reservas.is_empty() {
                  self.reservas.remove(&chave);
              }
          } else {
              println!("\nReserva não encontrada!");
          }
      } else {
          println!("\nReserva não encontrada!");
      }
  }
fn hash(numero: i32, mut tamanho_hash: i32) -> i32 {
    if tamanho_hash % 89 == 0 {
      //próximo número primo, se não todas as chaves vão ser mapeadas para o mesmo índice (0)
      tamanho_hash = 97;
    }
    let numero_reserva = ((numero << 5) ^ (numero >> 5)) * 89 ;
    return numero_reserva % tamanho_hash;
    //o número 89 seria um número primo aleatório, apenas para tentar reduzir mais ainda o número de colisões. (só para deixar explicado)
    //e o 5 do shift também é aleatório, mas é só para deixar o número mais específico 
  }
}

fn main() {
  let mut reservas = SistemaDeReserva::new();
  let mut tamanho_hash: i32 = 0;
  let mut num_adicoes: i32;

  //leitor do arquivo de carga de dados
  let file = File::open("src/leituraReservas.md");
  let reader = io::BufReader::new(file.as_ref().unwrap());
  for (index, line) in reader.lines().enumerate(){
    let line = match line {
        Ok(line) => line,
        Err(err) => {
            eprintln!("Erro ao ler o arquivo: {}", err);
            continue;
        }
    };
  if index == 0 {
        tamanho_hash = line.trim().parse().unwrap();
        continue;
    }
    
  let mut partes = line.trim().split(':');

  let num_reserva: i32 = partes.next().unwrap().trim().parse().unwrap();

  let dados: Vec<&str> = partes.next().unwrap().split(',').collect();
  let nome_hotel = dados[0].trim().to_string().to_lowercase();
  let num_quarto: i32 = dados[1].trim().parse().unwrap();
  let data_checkin = dados[2].trim().to_string();
  let data_checkout = dados[3].trim().to_string();
    
    let reserva = Reserva::new(num_reserva, nome_hotel, num_quarto, data_checkin, data_checkout);
    let chave = SistemaDeReserva::hash(num_reserva, tamanho_hash);
    
    reservas.adicionar_reserva_arquivo(chave, reserva);
  }
  //fim do leitor do arquivo de carga de dados

  //menu principal
  println!("\nBEM VINDO!");
  loop {
    let mut input = String::new();
    if reservas.reservas.is_empty() {
    println!("\nO que deseja fazer? \n1 - Adicionar reservas (N vezes) \n2 - Pesquisar reservas \n3 - Cancelar reserva \n4 - Sair");
    } else {
      println!("\nO que deseja fazer? \n1 - Adicionar mais reservas (N vezes) \n2 - Pesquisar reservas \n3 - Cancelar reserva \n4 - Sair");
    }
    io::stdin().read_line(&mut input).unwrap();
    let opcao: i32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Número digitado inválido!");
            continue;
        }
    };

  if opcao == 1 {
    //adicionar reservas
    println!("\nDigite o número de reservas que deseja adicionar (N): ");
      let mut input = String::new();
      io::stdin().read_line(&mut input).unwrap();
      num_adicoes = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
          println!("\nNúmero digitado inválido!");
          continue;
        }
      };
    
    println!("\nModelo de Resposta*: <NÚMERO_RESERVA>:<NOME_HOTEL>,<NÚMERO_QUARTO>,<DATA_CHECKIN>,<DATA_CHECKOUT> ");
    println!("\nPara formalização, escreva as datas no formato dd-mm-aa");
    println!("Exemplo: 01-01-24");
    
    println!("\nDigite as informações requeridas da reserva no espaço a seguir!");
    
    for i in 0..num_adicoes{
      println!("\nRESERVA {}: ",i+1);
      let mut entrada = String::new();
      io::stdin().read_line(&mut entrada).unwrap();
    
      let mut partes = entrada.trim().split(':');
      
      let num_reserva: i32 = partes.next().unwrap().trim().parse().unwrap();
    
      let dados: Vec<&str> = partes.next().unwrap().split(',').collect();
      let nome_hotel = dados[0].trim().to_string().to_lowercase();
      let num_quarto: i32 = dados[1].trim().parse().unwrap();
      let data_checkin = dados[2].trim().to_string();
      let data_checkout = dados[3].trim().to_string();

      let reserva = Reserva::new(num_reserva, nome_hotel, num_quarto, data_checkin, data_checkout);
      let chave = SistemaDeReserva::hash(num_reserva, tamanho_hash);
      
      reservas.adicionar_reserva(chave, reserva);
    }
    
  } else if opcao == 2 {
    //pesquisar reservas
    if reservas.reservas.is_empty() == true {
      println!("\nNão há reservas salvas até o momento!");
      continue;
    } else {
      println!("\nDigite o número da reserva sob interesse: ");
      let mut input = String::new();
      io::stdin().read_line(&mut input).unwrap();
      let num_reserva:i32 = input.trim().parse().unwrap();
      
      let chave = SistemaDeReserva::hash(num_reserva, tamanho_hash);
      
      reservas.detalhes_reserva(chave);
    }
  } else if opcao == 3 {
    //remover reservas
    if reservas.reservas.is_empty() == true {
      println!("\nNão há reservas salvas até o momento!");
      continue;
    } else {
      println!("\nModelo de Resposta*: <NÚMERO_RESERVA>:<NOME_HOTEL>,<NÚMERO_QUARTO>,<DATA_CHECKIN>,<DATA_CHECKOUT> ");
      println!("\nPara formalização, escreva as datas no formato dd-mm-aa");
      println!("Exemplo: 01-01-24");
      
      println!("\nCaso não lembre de alguma informação, proucure pela reserva no campo 'Pesquisar reservas', utilizando o código(número da reserva) que foi fornecido à reserva!");
      println!("\nDigite as informações requeridas da reserva em questão no espaço a seguir:");
      
      let mut entrada = String::new();
      io::stdin().read_line(&mut entrada).unwrap();

      let mut partes = entrada.trim().split(':');

      let num_reserva: i32 = partes.next().unwrap().trim().parse().unwrap();

      let dados: Vec<&str> = partes.next().unwrap().split(',').collect();
      let nome_hotel = dados[0].trim().to_string().to_lowercase();
      let num_quarto: i32 = dados[1].trim().parse().unwrap();
      let data_checkin = dados[2].trim().to_string();
      let data_checkout = dados[3].trim().to_string();

      let reserva = Reserva::new(num_reserva, nome_hotel, num_quarto, data_checkin, data_checkout);
      let chave = SistemaDeReserva::hash(num_reserva, tamanho_hash);

      reservas.remover_reserva(chave ,&reserva);
    }
  } else if opcao == 4 {
    //sair
    println!("Saindo...");
    break;
  } else {
    println!("\nNúmero digitado inválido!");
    continue;
    }
  }
  //imprime as reservas
    for (&chave, lista_reservas) in &reservas.reservas {
      println!("\nChave: {}", &chave);
      for reserva in lista_reservas {
          println!("Reserva: {:?}", reserva);
      }
  }
}