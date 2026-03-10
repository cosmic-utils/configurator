

si on a:

#[serde(default)]
A {
    x: String
    b: B
}

#[serde(default)]
B {
    x2: String
}


impl Default for A {

    A {
        x: "hello1",
        b: B {
            x2: "hello2"
        }
    }
}

impl Default for B {

    B {
        x2: "hello3"
    }
}


quel devrait être le default pour B ?


cas 1: {

    B {
        x2: "hello2"
    }


    pour qu'il soit pris en compte, il faut:
    
    que B ne soit pas modifié

}



cas 2: {
    B {
        x2: "hello3"
    }


    pour qu'il soit pris en compte, il faut:

    que B soit modifié
}


--------------------------------------------


- regle 1: on stock default que quand le schema le definie pour le node en question
- 


si on set modified pour tous les enfants:
- on pouvait être dans un default d'un array, le forcant à écrire sa valeur
- ca aurait put être éviter, si on force un schema a ne pas avoir de default conflictuel.

- si c'est le cas, on peut definir les default simplement

et est ce que ca rend set modified pour tous les enfants safe ?

: non, il peut y avoir une struct qui avait un default definie plus haut qui ne sera plus definie.


contraite:
- tous doit avoir un default
- chaque default doit être identique