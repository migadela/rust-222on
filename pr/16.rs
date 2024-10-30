fn main() {
    let mut found = false;
    for m in 1..9 {
        for u in 1..9 {
            for x in 1..9 {
                for a in 1..9 {
                    for s in 1..9 {
                        for l in 1..9 {
                            for o in 1..9 {
                                for n in 1..9 {
                                    if m != u && m != x && m != a && m != s && m != l && m != o && m != n &&
                                       u != x && u != a && u != s && u != l && u != o && u != n &&
                                       x != a && x != s && x != l && x != o && x != n &&
                                       a != s && a != l && a != o && a != n &&
                                       s != l && s != o && s != n &&
                                       l != o && l != n &&
                                       o != n {

                                        let muxa = 1000 * m + 100 * u + 10 * x + a;
                                        let slon = 1000 * s + 100 * l + 10 * o + n;

                                        if muxa * a == slon {
                                            println!("  {}", muxa);
                                            println!("x   {}", a);
                                            println!("  ----");
                                            println!("  {}", slon);
                                            found = true;
                                        }
                                    }
                                    if found {
                                        break;
                                    }
                                }
                                if found {
                                    break;
                                }
                            }
                            if found {
                                break;
                            }
                        }
                        if found {
                            break;
                        }
                    }
                    if found {
                        break;
                    }
                }
                if found {
                    break;
                }
            }
            if found {
                break;
            }
        }
        if found {
            break;
        }
    }
}
