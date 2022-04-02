#[wasm_bindgen]
pub fn CheckPrime(s: &JsValue) {
    let mut input: String = s.as_string().unwrap();
    if(is_prime(input)){
        alert("Input is Prime");
    }
    else{
        alert("Input is Prime");
    }
}

pub fn is_prime(s: String)->bool
{
// add your code here:
return true;
}