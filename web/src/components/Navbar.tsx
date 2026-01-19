import irisImage from "../assets/iris.png";

const Navbar = () => (
  <nav className="border-b border-iris-purple px-6 md:px-12 py-5 flex items-center justify-between sticky top-0 bg-iris-black z-50">
    <div
      className="flex items-center gap-2 cursor-pointer"
      onClick={() => window.scrollTo({ top: 0, behavior: "smooth" })}
    >
      <img src={irisImage} alt="" width={100} />
      <span className="font-black text-2xl tracking-tighter uppercase font-sans">
        Iris
      </span>
    </div>
    <div className="hidden md:flex items-center gap-10 text-[10px] font-mono font-bold tracking-[0.2em] uppercase text-zinc-500">
      <a href="#workflow" className="hover:text-iris-purple transition-colors">
        Workflow
      </a>
      <a
        href="#playground"
        className="hover:text-iris-purple transition-colors"
      >
        Playground
      </a>
      <a href="#docs" className="hover:text-iris-purple transition-colors">
        Implementation
      </a>
    </div>
    <button className="bg-iris-purple text-iris-black text-[10px] font-mono font-bold uppercase px-6 py-2 border border-iris-purple hover:bg-iris-black hover:text-iris-purple transition-all">
      <a href="#docs" className="hover:text-iris-purple transition-colors">
        Start Building
      </a>
    </button>
  </nav>
);

export default Navbar;
