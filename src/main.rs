mod fst;

use std::fs::File;
use crate::fst::std::VectorFst;

fn traverse<'a, FST : fst::std::Fst<'a>>(g : &'a FST){
    use fst::std::*;

    let mut siter = StateIterator::new(g);

    while !siter.Done() {
        let state = siter.Value();
        let mut aiter = ArcIterator::new(g,state.clone());
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

fn main() -> std::io::Result<()> {
    use crate::fst::std;
    let mut g = std::VectorFst::new();
    {
        let s1 = g.AddState();
        let s2 = g.AddState();
        let s3 = g.AddState();

        g.AddArc(s1,
                 std::Arc {
                     ilabel: 0,
                     olabel: 0,
                     weight: std::Weight::One(),
                     nextstate: s2
                 });

        g.AddArc(s2,
                 std::Arc {
                     ilabel: 1,
                     olabel: 1,
                     weight: std::Weight::One(),
                     nextstate: s3
                 });


        g.SetStart(s1);
        g.SetFinal(s3, std::Weight::new(5.));
    }

    traverse(&g);

    println!("try const!");

    let g_const = std::ConstFst::new(&g);

    traverse(&g_const);


    println!("try read/write!");


    {
        let mut file = File::create("fst.bin")?;
        g.Write(&mut file)?;
    }

    {
        let mut file = File::open("fst.bin")?;
        VectorFst::Read(&mut file).and_then(|fst|{
           traverse(&fst);
            return Ok(());
        })?;
    }

    return Ok(());

}
