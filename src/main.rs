use rs_graph::{Net, traits::*, EdgeVec};
use rs_graph::mst::kruskal;
use rs_graph::string::{Data, from_ascii};


// ALGORITMO DE KRUSKAL EM RUST
// MARCOS MORETTO
// ALUNO: RAFAEL IGNAULIN
// UNOCHAPECO - 2021

fn main(){

    let Data { graph, weights, nodes } = from_ascii::<Net>(r"
        a--------7------b-------8-------c
         \             / \             /
          \           /   \           /
           \         /     \         /
            5       9       7       5
             \     /         \     /
              \   /           \   /
               \ /             \ /
                d------15-------e
                 \             /   
                  \           /
                   \         /
                    6       8
                     \     /
                      \   /
                       \ /
                        f 
    ").unwrap();
    
    let a = nodes[&'a'];
    let b = nodes[&'b'];
    let c = nodes[&'c'];
    let d = nodes[&'d'];
    let e = nodes[&'e'];
    let f = nodes[&'f'];
    
    // Rodar o algoritmo
    let weights = EdgeVec::new_from_vec(&graph, weights);
    let arvore = kruskal(&graph, |e| weights[e]);
    
    // Testar os resultados
    let mut soma = 0;

    for &e in &arvore { 
        println!("Peso Menor: {}", weights[e]);
        soma += weights[e]; }

    assert_eq!(soma, 30);
    println!("A soma devera ser {}", soma);

    let mut arvore = arvore.into_iter()
    .map(|e| graph.enodes(e))
    .map(|(u,v)| if graph.node_id(u) > graph.node_id(v) { (v,u) } else { (u,v) })
    .collect::<Vec<_>>();

    arvore.sort_by_key(|&(u,v)| (graph.node_id(u), graph.node_id(v)));
    
    assert_eq!(arvore, vec![(a,b), (a, d), (b,e), (c,e), (d,f)]);

    println!("Achou a rota minima, kruskal finalizado!!");

        // SEQUENCIA FINAL 
        //
        // a--------7------b               c
        //  \               \             /
        //   \               \           /
        //    \               \         /
        //     5               7       5
        //      \               \     /
        //       \               \   /
        //        \               \ /
        //         d               e
        //          \                
        //           \           
        //            \         
        //             6       
        //              \     
        //               \   
        //                \ 
        //                 f 
        //
}
