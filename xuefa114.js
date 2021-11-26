'user strict';

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
    xf.insertAdjacentHTML('afterend', '<div> <iframe  id="aframe" scrolling="auto"></iframe></div>');
    xf.insertAdjacentHTML('beforeend', '<button id="btnAjax"> Click me.!</button> ');
    let btn = document.querySelector('div.g-hd');
    e = a_array[0];
    btn.onclick = function(){
	document.getElementById('aframe').setAttribute('src', e);
    };
    // btn.onclick = function(){
    // 	// 发送ajax 请求 需要 五步
    //     // （1）创建异步对象
    //     var ajaxObj = new XMLHttpRequest();
    //     // （2）设置请求的参数。包括：请求的方法、请求的url。
    //     ajaxObj.open('get', e);
    //     // （3）发送请求
    //     ajaxObj.send();
    //     //（4）注册事件。 onreadystatechange事件，状态改变时就会调用。
    //     //如果要在数据完整请求回来的时候才调用，我们需要手动写一些判断的逻辑。
    //     ajaxObj.onreadystatechange = function () {
    //         // 为了保证 数据 完整返回，我们一般会判断 两个值
    //         if (ajaxObj.readyState == 4 && ajaxObj.status == 200) {
    //             // 如果能够进到这个判断 说明 数据 完美的回来了,并且请求的页面是存在的
    //             // 5.在注册的事件中 获取 返回的 内容 并修改页面的显示
    //             console.log('数据返回成功');
    //             // 数据是保存在 异步对象的 属性中
    //             console.log(ajaxObj.responseText);
    //             // 修改页面的显示
    //         }
    //         for (const e of a_array) {
    // 		console.log(e);
    //         }
    // 	};
    // }
}
