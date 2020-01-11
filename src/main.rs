mod fst;
use fst::vector_fst;
use fst::fst_traits;
use fst::const_fst;
use crate::fst::std_arc::{StdArc, Weight};
use crate::fst::fst_traits::{BaseFst, IterableFst, StateIterator, ArcIterator};


fn traverse<'a, FST : fst_traits::IterableFst<'a,StdArc>>(g : &'a FST){
    use fst_traits::MakeStateIterator;
    use fst_traits::MakeArcIterator;

    let mut siter = MakeStateIterator(g);

    while !siter.Done() {
        let state = siter.Value();
        let mut aiter = MakeArcIterator(g,state.clone());
        while !aiter.Done() {
            let arc = aiter.Value();
            println!("{from} -> {to}, i:{input}/o:{output}, w : {weight}",
                     from=state,
                     to=arc.nextstate,
                     input=arc.ilabel,
                     output=arc.olabel,
                     weight=arc.weight.Value());
            aiter.Next();
        }
        println!("final: {state} -> {weight}",
                 state=state,
                 weight=g.Final(state).Value());
        siter.Next();
    }
}

fn main() {

    let mut g = vector_fst::VectorFst::new();
    {
        let s1 = g.AddState();
        let s2 = g.AddState();
        let s3 = g.AddState();

        g.AddArc(s1.clone(),
                 StdArc {
                     ilabel: 0,
                     olabel: 0,
                     weight: Weight::One(),
                     nextstate: s2.clone()
                 });

        g.AddArc(s2.clone(),
                 StdArc {
                     ilabel: 1,
                     olabel: 1,
                     weight: Weight::One(),
                     nextstate: s3.clone()
                 });


        g.SetStart(s1);
        g.SetFinal(s3, Weight::new(5.));
    }


    traverse(&g);

    println!("try const!");

    let g_const = const_fst::ConstFst::new(&g);
    traverse(&g_const);
}
