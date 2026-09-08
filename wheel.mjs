const PORT=9333,BASE='http://127.0.0.1:8080';const sleep=ms=>new Promise(r=>setTimeout(r,ms));
class S{constructor(ws){this.ws=ws;this.id=0;this.p=new Map();}
 static async open(u){const ws=new WebSocket(u);await new Promise((a,b)=>{ws.onopen=a;ws.onerror=b;});
  const s=new S(ws);ws.onmessage=e=>{const m=JSON.parse(e.data);if(m.id&&s.p.has(m.id)){s.p.get(m.id)(m);s.p.delete(m.id);}};return s;}
 send(m,p={}){const id=++this.id;this.ws.send(JSON.stringify({id,method:m,params:p}));return new Promise(r=>this.p.set(id,r));}
 async eval(e){const r=await this.send('Runtime.evaluate',{expression:e,returnByValue:true,awaitPromise:true});
   if(r.result?.exceptionDetails)throw new Error(JSON.stringify(r.result.exceptionDetails).slice(0,250));return r.result?.result?.value;}}
const l=await(await fetch(`http://127.0.0.1:${PORT}/json/list`)).json();
const s=await S.open(l.find(t=>t.type==='page').webSocketDebuggerUrl);
await s.send('Page.enable');await s.send('Runtime.enable');
await s.send('Emulation.setEmulatedMedia',{features:[{name:'prefers-reduced-motion',value:'no-preference'}]});
await s.send('Emulation.setDeviceMetricsOverride',{width:1440,height:900,deviceScaleFactor:1,mobile:false});
const READY=`(()=>{const b=getComputedStyle(document.body);const n=document.querySelector('.navbar-container');
  return !!n&&getComputedStyle(n).position==='fixed'&&b.marginLeft==='20px'&&!!document.querySelector('#hero');})()`;
await s.send('Page.navigate',{url:BASE+'/'});
for(let i=0;i<80;i++){await sleep(150);try{if(await s.eval(READY))break;}catch{}}await sleep(600);
console.log('snapType =', await s.eval(`getComputedStyle(document.documentElement).scrollSnapType`));
console.log('reducedMotion =', await s.eval(`matchMedia('(prefers-reduced-motion: reduce)').matches`));
const snapPoints = await s.eval(`(()=>{const cs=getComputedStyle(document.documentElement);
  const pad=parseFloat(cs.scrollPaddingTop)||0, vis=innerHeight-pad; const out={};
  for(const id of ['hero','aboutme','myworks','reachme']){const e=document.getElementById(id);
    const top=e.getBoundingClientRect().top+scrollY;
    const al=getComputedStyle(e).scrollSnapAlign;
    out[id]= al.startsWith('start')? Math.round(top-pad) : Math.round(top+e.offsetHeight/2-pad-vis/2);}
  return out;})()`);
console.log('expected snap positions:', JSON.stringify(snapPoints));
// simulate a real wheel gesture: several wheel ticks, then let it settle
for (const ticks of [3, 5, 7]) {
  await s.eval(`window.scrollTo({top:0,behavior:'instant'})`); await sleep(500);
  for (let i=0;i<ticks;i++){
    await s.send('Input.dispatchMouseEvent',{type:'mouseWheel',x:700,y:450,deltaX:0,deltaY:120,pointerType:'mouse'});
    await sleep(30);
  }
  await sleep(1500);
  const y=await s.eval(`Math.round(scrollY)`);
  const nearest=Object.entries(snapPoints).reduce((a,b)=>Math.abs(b[1]-y)<Math.abs(a[1]-y)?b:a);
  console.log(`  ${String(ticks).padStart(2)} wheel ticks -> settled y=${String(y).padEnd(5)} nearest snap "${nearest[0]}"@${nearest[1]} off by ${Math.abs(y-nearest[1])}px ${Math.abs(y-nearest[1])<=2?'SNAPPED':'NOT SNAPPED'}`);
}
process.exit(0);
