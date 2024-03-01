use binary_search_tree::BinarySearchTree;

pub fn frogs_and_mosquitoes(frogs: &mut Vec<(u32, u32)>, mosquitoes:&mut Vec<(u32, u32)>) {

    let mut frogs_tree = BinarySearchTree::new();
    let mut uneaten_mosq = BinarySearchTree::new();

    frogs.sort_by(|a, b| a.0.cmp(&b.0));
    let processed_frogs: Vec<(u32, u32)> = remove_overlaps(&frogs);

    for frog in processed_frogs {
        frogs_tree.insert(frog);
    }

    // 

    for mosquito in mosquitoes { 

        let mut frogs_to_remove = vec![];
        let mut frogs_to_update = vec![];
        let mut mosquitoes_to_remove = vec![];

        let tree_clone = frogs_tree.clone();
        let mut mosquito_query = mosquito.clone(); mosquito_query.1 += 1;   // to avoid perfect overlaps
        let opt_predecessor = tree_clone.predecessor(&mosquito_query);

        if let Some(frog) = opt_predecessor {

            let mut tongue_pos = frog.0 + frog.1;

            if tongue_pos >= mosquito.0 {                  // it is eatable

                tongue_pos += mosquito.1;

                // search for successor in uneaten mosquito and try to eat it 
                let mut opt_successor = uneaten_mosq.successor(frog);

                while opt_successor.is_some() && opt_successor.unwrap().0 <= tongue_pos {

                    let successor = opt_successor.unwrap();

                    tongue_pos += successor.1;

                    mosquitoes_to_remove.push(successor.clone());

                    opt_successor = uneaten_mosq.successor(successor);
                } 



                // remove overlaps with other frogs in the bst
                let mut opt_successor = tree_clone.successor(frog);

                // while overlap with others
                while opt_successor.is_some() && opt_successor.unwrap().0 <= tongue_pos {

                    let successor = opt_successor.unwrap();

                    if tongue_pos > successor.0 + successor.1 {
                        // total overlap
                        frogs_to_remove.push(successor.clone());
                    }

                    else {
                        let new_successor_pos = successor.0 + (tongue_pos - successor.0) + 1;
                        let new_successor_tongue = successor.1 - new_successor_pos;

                        frogs_to_update.push((successor, (new_successor_pos, new_successor_tongue)));
                    }

                    opt_successor = tree_clone.successor(successor);
                }

                // update eating frog
                frogs_to_update.push((frog, (frog.0, tongue_pos - frog.0)));
            }

            else {
                uneaten_mosq.insert(*mosquito);
            }
        }

        // finalize changes - this stage is separated because of the borrow checker controls! :)

        for frog in frogs_to_remove {
            frogs_tree.remove(&frog);
        }

        for (frog_old, frog_new) in frogs_to_update {
            frogs_tree.remove(frog_old);
            frogs_tree.insert(frog_new);
        }

        for mosquito in mosquitoes_to_remove {
            uneaten_mosq.remove(&mosquito);
        }

    }

    println!("{:?}", frogs_tree.into_sorted_vec());

}

fn remove_overlaps(frogs: &Vec<(u32, u32)>) -> Vec<(u32, u32)> {
    let mut p_frogs = vec![frogs[0]];
    let mut tounge_pos = frogs[0].0 + frogs[0].1;

    for i in 1..frogs.len() {
        let frog = frogs[i];

        if frog.0 > tounge_pos {
            tounge_pos = frog.0 + frog.1;
            p_frogs.push(frog);
        } else {
            let right_extr = frog.0 + frog.1;
            if right_extr <= tounge_pos {
                continue;
            } else {
                p_frogs.push((tounge_pos + 1, frog.0 + frog.1 - (tounge_pos + 1)));
                tounge_pos = frog.0 + frog.1;
            }
        }
    }

    p_frogs
}
