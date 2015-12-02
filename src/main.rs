                            use
                          std::io
                            ; fn
                           main()
                             {
                            let
                          mut a =
                       String::new()
                       ; io::stdin()
                    .read_line(&mut a)
                   .ok().expect("Failed
                      to read line")
                  ; let a: i32 = match a
                 .trim().parse() { Ok(num)
                     => num, Err(_) =>
                { panic!("not a number!"); }
               }; let w = a * 2 + 5; for x in
                    0..a { for y in 0..3
                  { for z in 0..a-y-x+1 {
                 print!(" ");  }  for z in
                    0..(x*2)+((y*2)+1) {
                print!("."); } println!("");
              } } for x in 0..w/5+1 { for z in
             0..w/3 {  print!(" ");  } for z in
                      0..w-(w/3+1)*2 {
                       print!("#"); }
                        println!("")
                             ;
                             }
                             }
