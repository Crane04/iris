import { CloudUpload, Cpu, Terminal } from "lucide-react";

const TechnicalWorkflow = () => {
  const steps = [
    {
      title: "Request",
      icon: <CloudUpload className="w-10 h-10 text-iris-purple" />,
      description:
        "Hospital sends POST /compare with target_url and people map.",
    },
    {
      title: "In-Memory Inference",
      icon: <Cpu className="w-10 h-10 text-iris-purple" />,
      description: "Pixels are converted to 128-bit vectors in RAM.",
    },
    {
      title: "Result",
      icon: <Terminal className="w-10 h-10 text-iris-purple" />,
      description:
        "Similarity scores are returned; RAM is cleared immediately.",
    },
  ];

  return (
    <section
      id="workflow"
      className="py-24 px-6 max-w-7xl mx-auto border-t border-iris-purple"
    >
      <div className="mb-16 text-center">
        <h3 className="text-iris-purple font-mono text-[10px] font-bold tracking-[0.5em] uppercase mb-4">
          Technical Workflow
        </h3>
        <h2 className="text-4xl md:text-5xl font-black uppercase tracking-tight">
          The "No-Storage" Pipeline
        </h2>
      </div>
      <div className="grid grid-cols-1 md:grid-cols-3 gap-0 border border-iris-purple">
        {steps.map((step, idx) => (
          <div
            key={idx}
            className="p-12 bg-iris-grey/20 border-b md:border-b-0 md:border-r last:border-r-0 border-iris-purple text-center group"
          >
            <div className="flex justify-center mb-8">{step.icon}</div>
            <h4 className="text-xl font-black uppercase mb-4 font-sans tracking-tight">
              {step.title}
            </h4>
            <p className="text-zinc-500 text-sm leading-relaxed font-sans">
              {step.description}
            </p>
          </div>
        ))}
      </div>
    </section>
  );
};

export default TechnicalWorkflow;
