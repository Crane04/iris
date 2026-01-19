import { useState } from "react";
import { Plus, Trash2 } from "lucide-react";
import post from "../api";

const APIPlayground = () => {
  const [targetUrl, setTargetUrl] = useState(
    "https://static.wikia.nocookie.net/amazingspiderman/images/3/33/Tobey_Maguire_Infobox.png/revision/latest/scale-to-width-down/535?cb=20240322015635",
  );
  const [people, setPeople] = useState([
    {
      name: "Maguire",
      image_url:
        "https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcQqiVCCW7eH5Q_8q4VULShU7O8QnOgp7Us2RBNhAlnesh2_iho_D1Toosuxj_x66J1w8ks&usqp=CAU",
    },
    {
      name: "Tom",
      image_url:
        "https://static.wikia.nocookie.net/marvelcinematicuniverse/images/2/2f/Tom_Holland.jpg/revision/latest/scale-to-width-down/1200?cb=20220213015022",
    },
  ]);
  const [isLoading, setIsLoading] = useState(false);
  const [response, setResponse] = useState<any>({
    matches: [
      { name: "Patient_992", probability: 98.0 },
      { name: "Patient_415", probability: 14.2 },
    ],
  });

  const addPerson = () => {
    setPeople([...people, { name: "", image_url: "" }]);
  };

  const removePerson = (index: number) => {
    setPeople(people.filter((_, i) => i !== index));
  };

  const updatePerson = (
    index: number,
    field: "name" | "image_url",
    value: string,
  ) => {
    const updated = [...people];
    updated[index][field] = value;
    setPeople(updated);
  };

  const handleExecute = async () => {
    try {
      setIsLoading(true);
      const matches = await post("compare", { target_url: targetUrl, people });

      setResponse({ matches });
    } catch (error) {
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <section id="playground" className="py-24 px-6 max-w-7xl mx-auto">
      <div className="mb-12 border-l-4 border-iris-purple pl-6">
        <h2 className="text-5xl md:text-6xl font-black uppercase tracking-tighter mb-2 font-sans">
          REST API Playground
        </h2>
        <p className="text-zinc-500 font-mono text-[10px] font-bold tracking-widest uppercase">
          Live Demo Dashboard
        </p>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-2 gap-0 border border-iris-purple">
        {/* Left Input Pane */}
        <div className="p-8 md:p-12 bg-iris-black border-b lg:border-b-0 lg:border-r border-iris-purple">
          <div className="space-y-8">
            <div className="space-y-2">
              <label className="text-[10px] font-mono font-bold tracking-widest text-iris-purple uppercase">
                target_url
              </label>
              <input
                value={targetUrl}
                onChange={(e) => setTargetUrl(e.target.value)}
                className="w-full bg-iris-grey border border-iris-purple p-4 font-mono text-xs focus:bg-iris-purple/5 outline-none text-white"
                placeholder="The emergency capture URL"
              />
            </div>

            <div className="space-y-4">
              <div className="flex items-center justify-between">
                <label className="text-[10px] font-mono font-bold tracking-widest text-iris-purple uppercase">
                  people (comparison list)
                </label>
                <button
                  onClick={addPerson}
                  className="text-iris-purple hover:text-white flex items-center gap-1 text-[10px] font-mono font-bold uppercase"
                >
                  <Plus className="w-3 h-3" /> Add Person
                </button>
              </div>

              <div className="space-y-4 max-h-64 overflow-y-auto pr-2 custom-scrollbar">
                {people.map((person, idx) => (
                  <div
                    key={idx}
                    className="p-4 border border-zinc-800 bg-iris-grey/30 relative group"
                  >
                    <button
                      onClick={() => removePerson(idx)}
                      className="absolute top-2 right-2 text-zinc-600 hover:text-red-500"
                    >
                      <Trash2 className="w-3 h-3" />
                    </button>
                    <div className="grid grid-cols-2 gap-4">
                      <div className="space-y-1">
                        <span className="text-[8px] font-mono text-zinc-600 uppercase">
                          name
                        </span>
                        <input
                          value={person.name}
                          onChange={(e) =>
                            updatePerson(idx, "name", e.target.value)
                          }
                          className="w-full bg-iris-black border border-zinc-800 p-2 font-mono text-[10px] focus:border-iris-purple outline-none"
                        />
                      </div>
                      <div className="space-y-1">
                        <span className="text-[8px] font-mono text-zinc-600 uppercase">
                          image_url
                        </span>
                        <input
                          value={person.image_url}
                          onChange={(e) =>
                            updatePerson(idx, "image_url", e.target.value)
                          }
                          className="w-full bg-iris-black border border-zinc-800 p-2 font-mono text-[10px] focus:border-iris-purple outline-none"
                        />
                      </div>
                    </div>
                  </div>
                ))}
              </div>
            </div>

            <button
              onClick={handleExecute}
              disabled={isLoading}
              className="w-full bg-iris-purple text-iris-black font-mono font-bold uppercase py-5 flex items-center justify-center gap-2 hover:bg-white transition-all disabled:opacity-50"
            >
              {isLoading ? "Analyzing..." : "POST /compare"}
            </button>
          </div>
        </div>

        {/* Right Output Pane */}
        <div className="bg-iris-black flex flex-col min-h-[400px]">
          <div className="p-4 border-b border-iris-purple bg-iris-grey/50 flex items-center justify-between">
            <span className="text-[10px] font-mono font-bold tracking-widest text-zinc-400 uppercase">
              Output: Match Result JSON
            </span>
            <div className="flex gap-1.5">
              <div className="w-1.5 h-1.5 bg-iris-purple"></div>
              <div className="w-1.5 h-1.5 bg-iris-purple"></div>
            </div>
          </div>
          <div className="flex-1 p-8 md:p-12 overflow-auto font-mono text-sm">
            <pre className="text-iris-purple">
              {JSON.stringify(response, null, 2)}
            </pre>
          </div>
        </div>
      </div>
    </section>
  );
};

export default APIPlayground;
