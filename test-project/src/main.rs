/*--------------------------------------------------------------------------------------------------------------
 * Copyright (c) Microsoft Corporation. All rights reserved.
 * Licensed under the MIT License. See https://go.microsoft.com/fwlink/?linkid=2090316 for license information.
 *-------------------------------------------------------------------------------------------------------------*/
pub struct MultiUse<D : Copy> {
    uses : usize,
    data : Box<D>
}
impl<D : Copy> MultiUse<D> {
    fn new(amout : usize, value: D ) -> Self {
        MultiUse { uses: (amout), data: Box::new(value) }
    }
    fn open(&mut self) -> D {
        if self.uses == 0 {
            panic!("you have used this variable to many times. You must use the revive method/nin this MultiUse");}
        self.uses -= 1;
        *self.data.clone()
    }
    fn revive(&mut self,lives : usize) {
        self.uses += lives
    }
    fn set(&mut self,data : D) {
        self.data = Box::new(data)
    }
}
