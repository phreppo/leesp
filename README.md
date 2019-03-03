# leesp
Lisp interpreter

----
Possible solutions for che cons cell:
. Cons(Rc<Cell>, Rc<Cell>), : everything is very clear and multiple references to the same cell are permitted, but very low performances. creating a new cons cell you must move
. Cons()
. Much more
. approfondire differenza fra box e referenza

cose discutibili di rust
. non mi e piaciuto il sistema di testing
. mi e piaciuto che il sistema di building e il linguaggio siano una cosa unica, con repo di packages annesse. La filosofia e che sia difficile comunque scrivere in rust: usa cose gia scritte e testate come hai fatto il parser. esiste anche un parser completamente specializzato per list che contiene gia anche i tipi, ma io volevo usare una mia implementazione. questo mi ha permesso tipo di evitare quei brutti errori fatali che ho incontrato nel parser ad un certo punto del progetto, nascosti in poche righe di codice C
. mi sono piaciuti gli algebraic data types
. il supporto builtin ad un sacco di cose
. i tratti e l astrazione zero cost
. test anche sulla docuemntazione
. cosa non piaciuta: i test dentro il codice (cita clean code)
. libro bibbia
. loop keyword che ha senso per cose pesanti
. best practice nel libro bibbia (per programmi a linea di comando)
. supporto utf 8
. supporto ai colori in liena con la filosofia, perche e multipiattaforma (invece quello del C di mettere dei caratteri e un po rozzo)
. il longjump viene evitato
. la differenza per fare un car o un cdr imopressionante fra rust e c
. controlla le variabili non usate, ti controlla anche come le chiami e che rispetti le regole sintattiche
. perche secondo me usa ancora i ;: c e la necessita di andare a capo in quanto un linguaggio motlo verboso
. errori se tutto il match non coperto
. invece di un milione di flag per l ottimizzazione passate alc ompilatore, basta usare cargo e la sua modalita per fare cose cool
. tokei e stata fatta con questo: adatto a applicazioni cli
. considerazioni: l interprete va N times piu lento, ma alla fine non mi ha mai dato segfault folli