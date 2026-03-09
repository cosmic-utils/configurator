si on a un config du system:


A {
    x: "from_system"
}

et dans home:

A {
    y: "from_home"
}

et on ecrit 

A {
    z: "from app"
}


le resultat doit être:


A {
    y: "from_home"
    z: "from_app"
}