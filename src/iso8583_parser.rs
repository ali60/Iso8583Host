use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs::File;
use std::str::FromStr;
use serde_xml_rs;
use std::i32;

#[derive(Clone,Deserialize, Debug)]
pub struct iso_transactions {
    version: String,
    #[serde(rename="transaction")]
    transactions: Vec<transaction>,
}


#[derive(Clone,Deserialize, Debug)]
pub struct transaction {
    mti: String,
    #[serde(rename="field")]
    fields: Vec<field>,
}


#[derive(Clone,Deserialize, Debug)]
pub struct field {
    num: String,
    format: String,
    length: String,
    value: String,
}

 

//check if field number is set in the bitmap
fn is_set(bitmap: String,field: i32) ->bool
{
	let mut  i =(field-1)/8;
	i = i*2;
	let j = (field-1)%8;
	let part: String = bitmap.chars().skip(i as usize).take(2).collect();  
	let s = i32::from_str_radix(&part, 16).unwrap();
	let mut a:i32 = 128;
	a = a >> j;
	if (a & s) >0 {
		return true;
    }
	return false;
}


fn get_field_format(field_no:String ,message_format: transaction)-> field
{
    let local :transaction = message_format.clone();
    let mut ret:field = field {num:String::new(),format:String::new(),length:String::new(),value:String::new()} ;
	for fl in local.fields {
		if field_no == fl.num {
           ret = fl.clone();
		   break;
		}
		
	}
	ret

} 


//get iso8583 field information from XML
fn extract_field(index:i32,message: String,field_record: field)-> (String,usize)
{
   let mut ret:String = String::new();
   let mut len:usize = 0;
   let mut field_len:usize = 0;
   let mut i:usize = index as usize;
   match field_record.format.as_str()
   {
//data format LLVAR and LLLVAR means variable length
     "LLVAR" =>{
			field_len=2;
	        let temp: String= message.chars().skip(i).take(field_len).collect();
			len = FromStr::from_str(&temp).unwrap();
			println!("LLVAR field,len={}",len);
			i=i+field_len;
			}
     "LLLVAR" =>{
			field_len=3;
	        let temp: String= message.chars().skip(i).take(field_len).collect();
			len = FromStr::from_str(&temp).unwrap();
			i=i+field_len;
			}
     "NUMERIC" =>{
	        len= field_record.length.trim().parse().expect("Wanted a number");}
     "ALPHA" =>{
	        len= field_record.length.trim().parse().expect("Wanted a number");}
     "DATE4" =>{
	        len= field_record.length.trim().parse().expect("Wanted a number");}
     "DATE6" =>{
	        len= field_record.length.trim().parse().expect("Wanted a number");}
     "DATE10" =>{
	        len= field_record.length.trim().parse().expect("Wanted a number");}
     "TIME" =>{
	        len= field_record.length.trim().parse().expect("Wanted a number");}
	 "AMOUNT" =>{
	        len= field_record.length.trim().parse().expect("Wanted a number");}
	 _ => panic!("error")
   }
   ret = message.chars().skip(i).take(len).collect();
   return (ret , len + field_len);
}

fn parse_incoming_to_umf(input_message: String ,message_format: &transaction  )
{
	let mti = input_message[..4].to_owned();
	let mut index:i32=4;
	let bitmap: String = input_message.chars().skip(index as usize).take(16).collect();
	index = index + 16;
	let is_sec_bitmap:bool = is_set(bitmap.to_string(),1);

	if is_sec_bitmap {
		index = index + 16;

	}
	//check if each field is present
	for x in 1..64 {
	    if is_set(bitmap.to_string(),x){
		        
			   let f = get_field_format(x.to_string(),message_format.clone());
				
			   let (field_val,u)=extract_field(index,input_message.to_string(),f);
			   println!("field {} value={}", x,field_val); 
			   index =  index + (u as i32);
		}
	}	
	if is_sec_bitmap {
		let sec_bitmap: String = input_message.chars().skip(12).take(16).collect();
		println!("second bitmap = {} ", sec_bitmap); 
		for x in 1..64 {
				println!("check field {} ..", x); 
			if is_set(sec_bitmap.to_string(),x){
			
				
				println!("field f {} is set", x); 
			}
		}	
	
	}
}

pub fn parse_request(str_buffer: String)->transaction
{
	let  ret_val:transaction;
    println!("Request: {}", str_buffer);
	let mti = str_buffer[..4].to_owned();
    println!("MTI: {}", mti);

	let mut file = File::open("iso8583_message_format.xml").expect("config file not found");
	let mut contents = String::new();
	file.read_to_string(&mut contents)
		.expect("something went wrong reading the file");
	let iso :iso_transactions = serde_xml_rs::deserialize(contents.as_bytes()).unwrap();
	for iso_transaction in iso.transactions {
		println!("{}", iso_transaction.mti);
		if mti == iso_transaction.mti
		{
		    parse_incoming_to_umf(str_buffer.to_string(),&iso_transaction);
		    ret_val = iso_transaction.clone();
			return ret_val;
		}
    }
	panic!("MTI not found");
}

trait StringUtils {
    fn substring(&self, start: usize, len: usize) -> Self;
}

impl StringUtils for String {
    fn substring(&self, start: usize, len: usize) -> Self {
        self.chars().skip(start).take(len).collect()
    }
}

fn set_bit(bitmap: String,field: i32)->String
{
	let mut  i =(field-1)/8;
	i = i*2;
	let j = (field-1)%8;
	let mut part: String = bitmap.chars().skip(i as usize).take(2).collect();  
	let mut s = i32::from_str_radix(&part, 16).unwrap();
	let mut a:i32 = 128;
	a = a >> j;
	s = s | a;
	part = format!("{:02x}", s);
	let mut ret :String = String::new();
	
	if i>0 {
		ret = bitmap.substring(0,i as usize);
	}
	
	ret.push_str(&part);
	if i < 16 {
	let temp = bitmap.substring( (i+2) as usize,(14-i) as usize);
	ret.push_str(&temp);
	}
	ret

}

fn generate_transaction(request_message: &transaction , response_message_format: &transaction  )-> String
{
    //bitmap is initially all zero
	//bitmap format is hex string
    let mut bitmap:String = "0000000000000000".to_string();
    let mut body:String = String::new();
    let mut gen_str:String = String::new();
    let local :transaction = response_message_format.clone();	
	gen_str.push_str(&local.mti);
	for f in local.fields
	{
	   if f.value.is_empty() == false {
		let  n = FromStr::from_str(&f.num).unwrap();
		
		bitmap = set_bit(bitmap,n);
		let mut slen:String = String::new();
		if f.format == "LLVAR"
		{
		  let count = f.value.chars().count();
		  slen = format!("{:02}", count);
		  body.push_str(&slen);
		}
		if f.format == "LLLVAR"
		{
		  let count = f.value.chars().count();
		  slen = format!("{:03}", count);
		  body.push_str(&slen);
		}
		if f.value == "ECHO"{
		   let req_field = get_field_format(f.num.clone(),request_message.clone());
		   println!("echo value={}",req_field.value);
		   body.push_str(&req_field.value);
		}
		else{
		   body.push_str(&f.value);
		}
		println!("field {} value = {}",f.num,f.value);
	   }
	}
	gen_str.push_str(&bitmap);
	gen_str.push_str(&body);
	gen_str
}

//generate ISO8583 reponse message ased on request transaction
pub fn generate_response(request_message: &transaction) ->String
{
    let mut response_string:String = String::new();
	let mut file = File::open("iso8583_message_format.xml").expect("config file not found");
	let mut contents = String::new();
	file.read_to_string(&mut contents)
		.expect("something went wrong reading the file");
	let iso :iso_transactions = serde_xml_rs::deserialize(contents.as_bytes()).unwrap();
	let mut resp_mti:i32 = request_message.mti.parse().unwrap();
	//response MTI is request MTI+10, for example for 0200, response is 0210
	resp_mti = resp_mti +10;
    let mut mti:String = resp_mti.to_string();
	mti.insert(0,'0');
	
	println!("generate response..");
	for iso_transaction in iso.transactions {
		if mti == iso_transaction.mti
		{
			println!("found MTI {}", iso_transaction.mti);
			response_string = generate_transaction(&request_message,&iso_transaction);
			return response_string;
		}
    }
   panic!("error generating response message");
}


