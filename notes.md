Ok donc de ce que jai compris de std::child

on lui dit au lancement, pendant le parsing :
let {liste des prog a lancer au demarrage} = Command::new("path vers le binaire ou lexecutable a lancer")
.arg("larg quon veut passer a notre commande")
.spawn() (donne vie au process -> demande a l os de creer une entree dans la table des process)
.expect("hihi oubli de push aye")

Pour recup le pid
let list_pid = list_enfant.id (boucle)

loop{
match list_pid.try_wait()
}

                        PArsing

Un vector dúne struct hashmap ca parait pas mal non ?
ca fait une string pour le nom du prog auquel est rattache toutes les infos presentes dans le .yaml.

                        POur le temps :

Std::time::Duration::from_secs(self.min_runtime)

# tourne dans le vide

1. parsing
2. lancement des process autostart
3. thread 'monitor'
   - Qui est mort ? Que faire ensuite ?
   - relancer les programs si jamais
   - ...
4. main thread == readline
   - parsing des commandes start|stop|status|exit
5. reload la config

# BIEN CHOISIR LES STRUCTURES

Mutex<Config> global variable

