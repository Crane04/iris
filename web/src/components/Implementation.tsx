import { useState } from "react";
import { Copy, Check } from "lucide-react";

const Implementation = () => {
  const [activeLang, setActiveLang] = useState("curl");
  const [copied, setCopied] = useState(false);

  const languages = [
    { id: "curl", name: "cURL" },
    { id: "rust", name: "Rust" },
    { id: "python", name: "Python" },
    { id: "javascript", name: "JavaScript" },
    { id: "go", name: "Go" },
  ];

  const codeExamples: Record<string, string> = {
    curl: `curl -X POST http://api.iris.dev/compare \\
  -d '{
    "target_url": "https://img.url/target",
    "people": [
      {
        "name": "Patient_ID_992",
        "image_url": "https://img.url/p992"
      }
    ]
  }'`,
    rust: `use serde::{Serialize, Deserialize};
use reqwest::Client;

#[derive(Serialize)]
struct CompareRequest {
    target_url: String,
    people: Vec<Person>,
}

#[derive(Serialize)]
struct Person {
    name: String,
    image_url: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let payload = CompareRequest {
        target_url: "https://img.url/target".into(),
        people: vec![Person {
            name: "Patient_ID_992".into(),
            image_url: "https://img.url/p992".into(),
        }],
    };

    let res = client.post("http://api.iris.dev/compare")
        .json(&payload)
        .send()
        .await?;

    println!("{:#?}", res.text().await?);
    Ok(())
}`,
    python: `import requests

payload = {
    "target_url": "https://img.url/target",
    "people": [
        {
            "name": "Patient_ID_992",
            "image_url": "https://img.url/p992"
        }
    ]
}

response = requests.post("http://api.iris.dev/compare", json=payload)
print(response.json())`,
    javascript: `const payload = {
  target_url: "https://img.url/target",
  people: [
    {
      name: "Patient_ID_992",
      image_url: "https://img.url/p992"
    }
  ]
};

fetch("http://api.iris.dev/compare", {
  method: "POST",
  headers: { "Content-Type": "application/json" },
  body: JSON.stringify(payload)
})
.then(res => res.json())
.then(console.log);`,
    go: `package main

import (
	"bytes"
	"encoding/json"
	"fmt"
	"net/http"
)

func main() {
	payload := map[string]interface{}{
		"target_url": "https://img.url/target",
		"people": []map[string]string{
			{
				"name":      "Patient_ID_992",
				"image_url": "https://img.url/p992",
			},
		},
	}
	
	jsonData, _ := json.Marshal(payload)
	resp, _ := http.Post("http://api.iris.dev/compare", "application/json", bytes.NewBuffer(jsonData))
	
	fmt.Println("Status:", resp.Status)
}`,
  };

  const handleCopy = () => {
    navigator.clipboard.writeText(codeExamples[activeLang]);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  return (
    <section
      id="docs"
      className="py-24 px-6 max-w-7xl mx-auto border-t border-iris-purple"
    >
      <div className="mb-12">
        <h3 className="text-3xl font-black uppercase tracking-tight mb-2 font-sans">
          Implementation Section
        </h3>
        <p className="text-iris-purple font-mono text-[10px] font-bold tracking-widest uppercase">
          Developer Proof: No SDK required. Pure REST implementation.
        </p>
      </div>

      <div className="border border-iris-purple bg-iris-black overflow-hidden">
        {/* Language Tabs */}
        <div className="flex border-b border-iris-purple overflow-x-auto no-scrollbar">
          {languages.map((lang) => (
            <button
              key={lang.id}
              onClick={() => setActiveLang(lang.id)}
              className={`px-8 py-4 font-mono text-[10px] font-bold uppercase tracking-widest transition-all border-r border-iris-purple last:border-r-0 whitespace-nowrap ${
                activeLang === lang.id
                  ? "bg-iris-purple text-iris-black"
                  : "text-zinc-500 hover:text-iris-purple hover:bg-iris-grey/50"
              }`}
            >
              {lang.name}
            </button>
          ))}
          <div className="ml-auto px-4 flex items-center">
            <button
              onClick={handleCopy}
              className="p-2 text-zinc-500 hover:text-white transition-colors flex items-center gap-2 font-mono text-[9px] uppercase font-bold"
            >
              {copied ? (
                <Check className="w-3 h-3 text-green-500" />
              ) : (
                <Copy className="w-3 h-3" />
              )}
              {copied ? "Copied" : "Copy"}
            </button>
          </div>
        </div>

        {/* Code Block */}
        <div className="p-8 md:p-12 overflow-x-auto bg-iris-grey/20 max-h-[500px] overflow-y-auto custom-scrollbar">
          <pre className="font-mono text-xs text-zinc-300 leading-relaxed">
            {codeExamples[activeLang]}
          </pre>
        </div>
      </div>
    </section>
  );
};

export default Implementation;
