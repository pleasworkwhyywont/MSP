

/*--------------------------------------------------------------------------------------------------------------
 * Copyright (c) Microsoft Corporation. All rights reserved.
 * Licensed under the MIT License. See https://go.microsoft.com/fwlink/?linkid=2090316 for license information.
 *-------------------------------------------------------------------------------------------------------------*/
macro_rules! creat_MultiUse {
    (set,$name:ident) => {
    fn set(&mut self , data : D) {
        self.data = Box::new(data)
    }
    };
    (new,$name:ident) => {
    fn new(amout : usize, value: D ) -> Self {
        $name { uses: (amout), data: Box::new(value) }
    }
    };
    (revive,$name:ident) => {
    fn revive(&mut self,lives : usize) {
            self.uses += lives
    }
    };
    (open,$name:ident) => {
        fn open(&mut self) -> D {
            if self.uses == 0 {
                panic!("you have ran out of the provided amout of uses")
            }
            self.uses -= 1;
            **&self.data
        }
    };
    ($name:ident _sep_$($methods:ident!),*) => {
    pub struct $name<D : Copy> {
        uses : usize,
        data : Box<D>
    }
    impl<D : Copy> $name<D> {
        $(creat_MultiUse!{$methods,$name})*
    }

    };
}
creat_MultiUse!{MutMultiUse _sep_ new!,revive!,set!,open!}
creat_MultiUse!{ConMultiUse _sep_ new!,revive!,open!}
creat_MultiUse!{NonRevMutMultiUse _sep_ new!,set!,open!}
creat_MultiUse!{NonRevConMultiUse _sep_ new!,open!}