use std::time::SystemTime;

// Transacao representa uma transação na blockchain
pub struct Transacao {
    remetente:    String,
    destinatario: String,
    valor:        f64,     // Quantidade transferida
    taxa:         f64,     // Taxa paga pelo cambio
    nonce:        u64,     // Identificador unico da transacao
    dados:        Vec<u8>, // Dados de entrada do contrato inteligente
    v:            Vec<u8>, // Componente da assinatura digital
    r:            Vec<u8>, // Componente da assinatura digital
    s:            Vec<u8>, // Componente da assinatura digital
}

// Bloco representa um bloco na blockchain
pub struct Bloco {
    validador:           Validador,      // Dados de quem validou a transação
    indice:              u64,            // Numero sequencial do bloco na blockchain
    timestamp:           SystemTime,     // Horario da implementação do bloco
    nonce:               u64,            // Identificador unico do bloco
    dificuldade:         BigUint,        // Numero de participantes atual da rede
    hash_bloco_anterior: String,
    hash_raiz_estado:    String,         // Hash representando o estado completo da blockchain
    hash_transacoes:     String,         // Hash representando todas as transações incluídas no bloco
    hash_recibos:        String,         // Hash representando os recibos das transações
    transacoes:          Vec<Transacao>,
    adicionais:          String,         // Dados extras uteis
}

// Validador representa um validador no sistema Proof of Stake
// O validador é literalmente uma pessoa
pub struct Validador {
    endereco:        String,  // Endereço do validador
    participacao:    f64,     // Quantidade de moedas colocadas como "stake" (garantia) pelo validador
    dados_validador: Vec<u8>, // Dados adicionais do validador, se necessário
    recompensa:      String,  // Valor pago como recompensa pelo trabalho
}
