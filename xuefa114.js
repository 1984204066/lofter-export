// ==UserScript==
// @name         显示lofter.com图片的原图网址
// @namespace    https://saber.love/?p=4073
// @version      0.1
// @description  在lofter.com的文章页，显示图片的原图网址，方便下载。
// @author       雪见仙尊 xuejianxianzun
// @match        https://xuefa114.lofter.com/*
// @icon 		http://ssf91.lofter.com/favicon.ico
// @run-at		document-end
// ==/UserScript==

'user strict';
function fixedEncodeURIComponent (str) {
    return encodeURIComponent(str).replace(/[!'()*]/g, function(c) {
	return '%' + c.charCodeAt(0).toString(16);
    });
}

let a_urls = '';
Array.prototype.Exists=function(v){
    var b=false;
    for(var i=0;i<this.length;i++){
	if(this[i]==v){b=true;break;}
    }
    return b;
}
function myFunction() {
    console.log('clicked');
}

let a_array = new Array();
let a_elements = document.querySelectorAll('a');
if (a_elements.length > 0) {
    var xf_reg = /xuefa.*post/
    for (const e of a_elements) {
	let href = e.href;
	console.log(href);
	if (xf_reg.test(e) && !a_array.includes(href)) {
	    a_urls += e + '<br>';
	    a_array.push(href);
	}
    }
    a_urls = '<div>' + a_urls + '</div>';
    a_iframe = '<div> <iframe  id="aframe" scrolling="auto"></iframe></div>';
    console.dirxml(a_urls);
    console.log(a_array);

    let xf = document.querySelector('div.g-hd');
    // xf.insertAdjacentHTML('afterend', a_urls);
    //let div = '<div> <iframe  id="aframe" scrolling="auto"></iframe></div>';
    let div = '<div id="result"></div>';
    xf.insertAdjacentHTML('afterend',  div);
    xf.insertAdjacentHTML('beforeend', '<button id="btnAjax"> Click me.!</button> ');
    let btn = document.querySelector('div.g-hd');
    let eind = 0;
    // btn.onclick = function(){
    // 	document.getElementById('aframe').setAttribute('src', e);
    // };
    btn.onclick = function(){
	let e = a_array[eind];
	console.log(e);
	// 发送ajax 请求 需要 五步
	var idx = e.lastIndexOf('/');
	console.log(idx);
	var fid = "0";
	if (idx > 0) {
	    fid = e.slice(idx + 1);
	    console.log(fid);
	}
        // （1）创建异步对象
        var ajaxObj = new XMLHttpRequest();
        // （2）设置请求的参数。包括：请求的方法、请求的url。
        ajaxObj.open('get', e);
        // （3）发送请求
        ajaxObj.send();
        //（4）注册事件。 onreadystatechange事件，状态改变时就会调用。
        //如果要在数据完整请求回来的时候才调用，我们需要手动写一些判断的逻辑。
        ajaxObj.onreadystatechange = function () {
	    // 为了保证 数据 完整返回，我们一般会判断 两个值
	    if (ajaxObj.readyState == 4 && ajaxObj.status == 200) {
                // 如果能够进到这个判断 说明 数据 完美的回来了,并且请求的页面是存在的
                // 5.在注册的事件中 获取 返回的 内容 并修改页面的显示
                console.log('数据返回成功');
                // 数据是保存在 异步对象的 属性中
                console.log(ajaxObj.responseText);
                // 修改页面的显示
		var xhr = new XMLHttpRequest();
		xhr.open('post', 'http://127.0.0.1:7878/' + fid);
		// 如果想要使用post提交数据,必须添加此行
		xhr.setRequestHeader("Content-type", "application/x-www-form-urlencoded");
		let html = ajaxObj.responseText;
		let temp = document.getElementById('result');
		(temp.textContent != undefined ) ? (temp.textContent = html) : (temp.innerText = html);
		// 将数据通过send方法传递
		//var data = temp.innerHTML;
		var data = fixedEncodeURIComponent(html);
		console.log(data);
		xhr.send(data);
		// 发送并接受返回值
		xhr.onreadystatechange = function () {
		    // 这步为判断服务器是否正确响应
		    if (xhr.readyState == 4 && xhr.status == 200) {
			console.log('数据post成功');
			eind ++;
			if (eind >= a_array.length) {
			    alert("over!");
			}
		    }
		};
	    }
	};
    }
}
