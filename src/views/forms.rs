/*
 * @Author: plucky
 * @Date: 2022-10-15 00:32:59
 * @LastEditTime: 2023-06-27 08:05:23
 * @Description: 
 */

#![allow(non_snake_case)]

use dioxus::{prelude::*};

use crate::model::Params;
// use tracing::info;


pub fn view(cx: Scope)->Element{
    
    cx.render(rsx!{
        div {
            class: "flex justify-center items-center text-sm",
            // Form Elements
            Forms{}
            
        }
    })
}


fn Forms(cx: Scope)->Element{
    let params = use_ref(&cx, Params::new);
    let result = use_state(&cx, String::new);
    let public_key = use_state(&cx, ||include_str!("../../public_key.pem").to_string());

    let check_params = move || {
        params.with(|p| {
            result.set(p.generate_sign(public_key));
        });
       
    };
    
    cx.render(rsx!{
        div {
            class: "mt-6 max-w-2xl w-11/12  mx-auto ",// 上边距6，最大宽度2xl，宽度11/12，水平居中
            
            div {
                class: "p-4 bg-white rounded-md shadow-md",
                h4 {
                    class: "text-gray-600",
                    "Params"
                }
                div {
                    class: "grid grid-cols-3 gap-1",
                    // 填充数据
                    params.read().data.iter().enumerate().map(|(id,(key, value))|{
                        let k = key.clone();
                        let v = value.clone();
                        
                    rsx!(
                        div {
                            class: "flex col-span-1",
                            input {
                                class: "form-input ",
                                r#type: "text",
                                value: "{key}",
                                // disabled: format_args!("{}", id < 3 ),
                                oninput: move |e| {
                                    params.with_mut(|p|{
                                        p.update_params(id, e.value.clone(), v.clone());
                                    });
                                },
                                
                            }
                            span {
                                class: "text-gray-600 self-center m-1",
                                ":"
                            }
                        }
                        
                        div{
                            class: "flex col-span-2",
                        
                        input {
                            // id: "{id}",
                            class: "form-input",
                            r#type: "text",
                            // placeholder: "Enter your appid",
                            value: "{value}",
                            oninput: move |e| {
                                // info!("input: {:?} {:?}", k.clone(), v.clone());
                                params.with_mut(|p|{
                                    p.update_params(id, k.clone(), e.value.clone());
                                });
                            },
                        }
                        button {
                            class: "px-2 ml-2 mt-1 rounded-md bg-gray-500  text-gray-200 hover:bg-red-600",
                            // hidden: format_args!("{}", id < 3 ),
                            onclick: move |_|{
                                params.with_mut(|p|{
                                    p.remove_params(id);
                                });
                                
                            },
                            "Del"
                        }
                        }
                    )
                    })
                    
                }
                div {
                    class: "flex justify-end mt-2",
                    button {
                        r#type: "button",
                        class: "form-button",
                        onclick: move |_|{
                            // info!("onclick: {:?}", 1);
                            params.with_mut(|p|{
                                p.add_params("".into(), "".into());
                            });
                            
                        },
                        "Add param"
                    }
                }
                h4 {
                    class: "text-gray-600 mt-2",
                    "Public key"
                }
                textarea {
                    class: "text-sm w-full h-52 md:h-36 border border-gray-200 rounded-md",
                    value: "{public_key}",
                    oninput: move |e| {
                        public_key.set(e.value.clone());
                    },
                }
                
                div {
                    class: "flex justify-end mt-2",
                    button {
                        r#type: "button",
                        class: "form-button",
                        onclick: move |_|{
                            check_params();
                        },
                        "Sign"
                    }
                }
            
            }
            // 结果部位
            
            div {
                class: " mt-1 p-4 bg-white rounded-md shadow-md",
                h4 {
                    class: "text-gray-600",
                    "Result"
                }
                textarea {
                    class: "text-sm w-full h-52 md:h-44 border border-gray-200 rounded-md",
                    readonly: "true",
                    rows: "5",
                    // cols: "100",
                    value: "{result}",
                    
                }
            }
        
        }
    })
}

mod icons{
    
    

}