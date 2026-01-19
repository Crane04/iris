const Hero = () => (
  <section className="py-24 md:py-40 px-6 flex flex-col items-center text-center bg-iris-black">
    <div className="max-w-4xl">
      <h1 className="text-8xl md:text-[10rem] font-black tracking-[-0.05em] uppercase mb-4 leading-none font-sans">
        Iris
      </h1>
      <h2 className="text-iris-purple text-xl md:text-2xl font-mono font-bold tracking-tight mb-8 uppercase">
        The infrastructure layer for medical face comparison.
      </h2>
      <p className="text-zinc-400 text-lg md:text-xl max-w-2xl mx-auto mb-12 font-sans">
        High-speed, stateless biometric matching for hospital emergency systems.
        Identify unresponsive patients without storing a single byte of
        biometric data. Open and free for medical developers.
      </p>
      <div className="flex flex-col sm:flex-row gap-4 justify-center">
        <a
          href="#playground"
          className="min-w-[220px] bg-iris-purple text-iris-black font-mono font-bold py-4 px-8 uppercase text-xs border border-iris-purple hover:bg-iris-black hover:text-iris-purple transition-all flex items-center justify-center"
        >
          Open Playground
        </a>
        <a
          href="#docs"
          className="min-w-[220px] flex items-center justify-center bg-iris-black text-iris-purple font-mono font-bold py-4 px-8 uppercase text-xs border border-iris-purple hover:bg-iris-purple hover:text-iris-black transition-all"
        >
          View Docs
        </a>
      </div>
    </div>
  </section>
);

export default Hero;
