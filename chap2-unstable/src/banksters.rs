macro_rules! exchange {
    (Give $amount:literal to $name:ident) => (
        $name.add($amount)
    );
    (Take $amount:literal from $giver:ident) => (
        $giver.subtract($amount)
    );
    (Give $amount:literal from $giver:ident to $receiver:ident) => (
        $giver.subtract($amount);
        $receiver.add($amount)
    );
}
