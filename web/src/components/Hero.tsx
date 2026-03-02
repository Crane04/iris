import { useEffect, useState } from "react";

const Hero = () => {
  const [isVisible, setIsVisible] = useState(false);
  const [isLoaded, setIsLoaded] = useState(false);

  useEffect(() => {
    const loadTimer = setTimeout(() => {
      setIsLoaded(true);
    }, 100);

    const visibleTimer = setTimeout(() => {
      setIsVisible(true);
    }, 300);

    return () => {
      clearTimeout(loadTimer);
      clearTimeout(visibleTimer);
    };
  }, []);

  return (
    <section className="py-24 md:py-40 px-6 flex flex-col items-center text-center bg-iris-black overflow-hidden relative">
      <div
        className="absolute right-0 top-1/4 w-64 h-64 bg-iris-purple/10 rounded-full blur-3xl transition-all duration-2000"
        style={{
          opacity: isLoaded ? 0.3 : 0,
          transform: isLoaded
            ? "translateX(0) scale(1)"
            : "translateX(100px) scale(0.8)",
          transitionDelay: "500ms",
        }}
      />
      <div
        className="absolute left-0 bottom-1/4 w-96 h-96 bg-iris-purple/5 rounded-full blur-3xl transition-all duration-2000"
        style={{
          opacity: isLoaded ? 0.2 : 0,
          transform: isLoaded
            ? "translateX(0) scale(1)"
            : "translateX(-100px) scale(0.8)",
          transitionDelay: "700ms",
        }}
      />

      <div className="max-w-4xl relative z-10">
        <div className="relative mb-4">
          <h1
            className="text-8xl md:text-[10rem] font-black tracking-[-0.05em] uppercase leading-none font-sans relative z-10 transition-all duration-1000 ease-out"
            style={{
              opacity: isVisible ? 1 : 0,
              transform: isVisible
                ? "translateY(0) scale(1)"
                : "translateY(30px) scale(0.95)",
            }}
          >
            Iris
            <span className="absolute inset-0 text-iris-purple opacity-0 hover:opacity-20 transition-opacity duration-300 pointer-events-none select-none overflow-hidden">
              Iris
            </span>
          </h1>

          <div
            className="absolute -inset-8 md:-inset-12 bg-gradient-to-r from-iris-purple/10 via-transparent to-iris-purple/5 rounded-full blur-3xl transition-all duration-1500 ease-out"
            style={{
              opacity: isVisible ? 1 : 0,
              transform: isVisible ? "scale(1)" : "scale(0.8)",
              transitionDelay: "200ms",
              animation: "pulse 4s ease-in-out infinite",
            }}
          />
        </div>

        <h2
          className="text-iris-purple text-xl md:text-2xl font-mono font-bold tracking-tight mb-8 uppercase transition-all duration-800 ease-out"
          style={{
            opacity: isVisible ? 1 : 0,
            transform: isVisible ? "translateY(0)" : "translateY(20px)",
            transitionDelay: "400ms",
          }}
        >
          The infrastructure layer for medical face comparison.
        </h2>

        <p
          className="text-zinc-400 text-lg md:text-xl max-w-2xl mx-auto mb-12 font-sans transition-all duration-800 ease-out"
          style={{
            opacity: isVisible ? 1 : 0,
            transform: isVisible ? "translateY(0)" : "translateY(20px)",
            transitionDelay: "600ms",
          }}
        >
          High-speed, stateless biometric matching for hospital emergency
          systems. Identify unresponsive patients without storing a single byte
          of biometric data. Open and free for medical developers.
        </p>

        <div
          className="flex flex-col sm:flex-row gap-4 justify-center transition-all duration-800 ease-out"
          style={{
            opacity: isVisible ? 1 : 0,
            transform: isVisible ? "translateY(0)" : "translateY(20px)",
            transitionDelay: "800ms",
          }}
        >
          <a
            href="#playground"
            className="group min-w-[220px] bg-iris-purple text-iris-black font-mono font-bold py-4 px-8 uppercase text-xs border border-iris-purple hover:bg-iris-black hover:text-iris-purple transition-all duration-300 flex items-center justify-center relative overflow-hidden transform hover:scale-[1.02] active:scale-[0.98]"
          >
            <div className="absolute inset-0 bg-iris-purple transition-transform duration-500 ease-out group-hover:scale-x-100 scale-x-0 origin-left" />

            <span className="relative z-10 transition-all duration-300 group-hover:tracking-wider flex items-center gap-2">
              Open Playground
              <span className="transition-all duration-300 group-hover:translate-x-1 group-hover:scale-110">
                →
              </span>
            </span>
          </a>

          <a
            href="#docs"
            className="group min-w-[220px] flex items-center justify-center bg-iris-black text-iris-purple font-mono font-bold py-4 px-8 uppercase text-xs border border-iris-purple hover:bg-iris-purple hover:text-iris-black transition-all duration-300 relative overflow-hidden transform hover:scale-[1.02] active:scale-[0.98]"
          >
            {/* Button background */}
            <div className="absolute inset-0 bg-iris-purple transition-transform duration-500 ease-out group-hover:scale-x-100 scale-x-0 origin-left" />

            {/* Button content */}
            <span className="relative z-10 transition-all duration-300 group-hover:tracking-wider flex items-center gap-2">
              View Docs
              <span className="transition-all duration-300 opacity-0 group-hover:opacity-100 group-hover:translate-x-1 group-hover:scale-110">
                →
              </span>
            </span>
          </a>
        </div>
      </div>

      {/* Additional floating elements */}
      <div
        className="absolute top-1/3 left-10 w-4 h-4 bg-iris-purple/30 rounded-full transition-all duration-3000"
        style={{
          opacity: isLoaded ? 1 : 0,
          transform: isLoaded ? "translateY(0)" : "translateY(-50px)",
          animation: "float 6s ease-in-out infinite",
          animationDelay: "1s",
        }}
      />
      <div
        className="absolute bottom-1/3 right-10 w-6 h-6 bg-iris-purple/20 rounded-full transition-all duration-3000"
        style={{
          opacity: isLoaded ? 1 : 0,
          transform: isLoaded ? "translateY(0)" : "translateY(50px)",
          animation: "float 8s ease-in-out infinite",
          animationDelay: "1.5s",
        }}
      />
    </section>
  );
};

export default Hero;
