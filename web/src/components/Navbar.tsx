import { useState } from "react";
import irisImage from "../assets/iris.png";

const Navbar = () => {
  const [isMenuOpen, setIsMenuOpen] = useState(false);

  const toggleMenu = () => {
    setIsMenuOpen(!isMenuOpen);
  };

  const closeMenu = () => {
    setIsMenuOpen(false);
  };

  const scrollToTop = () => {
    window.scrollTo({ top: 0, behavior: "smooth" });
    closeMenu();
  };

  const navLinks = [
    { id: "workflow", label: "Workflow" },
    { id: "playground", label: "Playground" },
    { id: "docs", label: "Implementation" },
  ];

  return (
    <>
      <nav className="border-b border-iris-purple px-6 md:px-12 py-5 flex items-center justify-between sticky top-0 bg-iris-black z-50">
        <div
          className="flex items-center gap-2 cursor-pointer"
          onClick={scrollToTop}
        >
          <img src={irisImage} alt="" width={100} />
          <span className="font-black text-2xl tracking-tighter uppercase font-sans">
            Iris
          </span>
        </div>

        {/* Desktop Navigation */}
        <div className="hidden md:flex items-center gap-10 text-[10px] font-mono font-bold tracking-[0.2em] uppercase text-zinc-500">
          {navLinks.map((link) => (
            <a
              key={link.id}
              href={`#${link.id}`}
              className="hover:text-iris-purple transition-colors"
            >
              {link.label}
            </a>
          ))}
        </div>

        <div className="hidden md:flex items-center gap-4">
          <button className="bg-iris-purple text-iris-black text-[10px] font-mono font-bold uppercase px-6 py-2 border border-iris-purple hover:bg-iris-black hover:text-iris-purple transition-all">
            <a href="#docs">Start Building</a>
          </button>
        </div>

        {/* Mobile Menu Button */}
        <button
          className="md:hidden flex flex-col items-center justify-center w-8 h-8 z-50"
          onClick={toggleMenu}
          aria-label={isMenuOpen ? "Close menu" : "Open menu"}
        >
          <span
            className={`w-6 h-0.5 bg-iris-purple transition-all duration-300 ${
              isMenuOpen ? "rotate-45 translate-y-1" : ""
            }`}
          />
          <span
            className={`w-6 h-0.5 bg-iris-purple my-1.5 transition-all duration-300 ${
              isMenuOpen ? "opacity-0" : "opacity-100"
            }`}
          />
          <span
            className={`w-6 h-0.5 bg-iris-purple transition-all duration-300 ${
              isMenuOpen ? "-rotate-45 -translate-y-1" : ""
            }`}
          />
        </button>

        {/* Mobile Menu */}
        <div
          className={`fixed inset-0 bg-iris-black/95 backdrop-blur-sm z-40 md:hidden transition-all duration-500 ease-in-out ${
            isMenuOpen
              ? "opacity-100 translate-y-0"
              : "opacity-0 -translate-y-full pointer-events-none"
          }`}
          onClick={closeMenu}
        >
          <div
            className={`absolute top-0 left-0 right-0 bottom-0 flex flex-col items-center justify-center transition-all duration-700 ease-out ${
              isMenuOpen
                ? "translate-y-0 opacity-100"
                : "translate-y-10 opacity-0"
            }`}
            onClick={(e) => e.stopPropagation()}
          >
            <div className="flex flex-col items-center space-y-10">
              {navLinks.map((link, index) => (
                <a
                  key={link.id}
                  href={`#${link.id}`}
                  className="text-2xl font-mono font-bold tracking-[0.2em] uppercase text-zinc-500 hover:text-iris-purple transition-all duration-300 transform hover:scale-105"
                  onClick={closeMenu}
                  style={{
                    transitionDelay: isMenuOpen ? `${index * 100}ms` : "0ms",
                  }}
                >
                  {link.label}
                </a>
              ))}

              <button
                className="mt-10 bg-iris-purple text-iris-black text-sm font-mono font-bold uppercase px-8 py-3 border border-iris-purple hover:bg-iris-black hover:text-iris-purple transition-all duration-300 transform hover:scale-105"
                onClick={() => {
                  closeMenu();
                  window.location.href = "#docs";
                }}
                style={{
                  transitionDelay: isMenuOpen
                    ? `${navLinks.length * 100}ms`
                    : "0ms",
                }}
              >
                Start Building
              </button>
            </div>
          </div>
        </div>
      </nav>
    </>
  );
};

export default Navbar;
