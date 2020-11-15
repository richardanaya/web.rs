
const fs = require('fs');
const source = String(fs.readFileSync("source.txt"));
let lines = source.split("\n")
let started = false;

function moduleName(n){
    n = n.replace("-"," ")
    n = n.replace("-"," ")
    n = n.replace("&","and")
  n= n.split(" ") 
  return n.map(x=>x.toLowerCase()).join("_");
}

function parseEmoji(n){
    if(n.indexOf("0023 FE0F 20E3") == 0){
        return {character:"#️⃣",name:"KEYCAP_HASH"};
    }
    let orig = n;
    n = n.replace(";","#")
    n = n.split("#");
    if(n[1].trim()!=="fully-qualified"){
        return null;
    }
    console.log(n)
    n = n.map(x=>x.trim());

    n = n[2].split(" ");
    n = n.map(x=>x.trim());
    const emoji = n[0];
    n.shift()
    n.shift()
    n = n.map(x=>x.toUpperCase())
    n = n.join("_")
    n = n.replace("*","ASTERISK");
    n = n.replace("(","");
    n = n.replace(")","");
    n = n.replace("1ST","FIRST");
    n = n.replace("2ND","SECOND");
    n = n.replace("3RD","THIRD");
    n = n.replace(":","");
    n = n.replace("&","AND");
    n = n.replace(".","");
    n = n.replace(".","");
    n = n.replace("'","");
    n = n.replace("’","");
    n = n.replace(",","");
    n = n.replace(",","");
    n = n.replace(",","");
    n = n.replace("-","_");
    n = n.replace("-","_");
    n = n.replace("“","_");
    n = n.replace("”","_");
    n = n.replace("!","");
    n = n.replace("Ñ","N");
    n = n.replace("Å","A");
    n = n.replace("É","E");
    n = n.replace("Ã","A");
    n = n.replace("Í","I");
    n = n.replace("Ç","C");
    n = n.replace("Ô","O");
    
    return {character:emoji,name:n};
}

let groups = [];
for(const l of lines){
    if(!started && l.indexOf("# group") == 0){
        started = true;
    }
    if(started){
        if(l.trim().length == 0 || l == "#EOF"){
            continue;
        }
        if(l.indexOf("#") == 0){
            if(l.indexOf("# group") == 0){
                let group = moduleName(l.substr(9));
                groups.push({name:group,subgroups:[]});
            }

            if(l.indexOf("# subgroup") == 0){
                let subgroup = moduleName(l.substr(12));
                groups[groups.length-1].subgroups.push({name:subgroup,emoji:[]});
            }
        } else {
            let e = parseEmoji(l);
            if(e!=null){
                groups[groups.length-1].subgroups[groups[groups.length-1].subgroups.length-1].emoji.push(e);
            }
        }
    }
}

let rust_mod = groups.map(g=>{
    return `pub mod ${g.name} {
        ${g.subgroups.map(s=>{
            return `pub mod ${s.name} {
                ${s.emoji.map(e=>{
                    return `pub const ${e.name}: &'static str = "${e.character}";`
                }).join("\n")}
            }`
        }).join("\n\n")}
    }`
}).join("\n\n");

fs.writeFileSync("src/lib.rs",rust_mod);