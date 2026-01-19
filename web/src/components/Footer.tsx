import irisImage from "../assets/iris.png";

const Footer = () => {
  return (
    <footer className="py-20 px-6 border-t border-iris-purple bg-iris-black text-center">
      <div className="max-w-7xl mx-auto flex flex-col items-center gap-8">
        <div className="flex items-center gap-2">
          <img src={irisImage} alt="" width={100} />
          <span className="font-mono text-[10px] font-bold tracking-[0.3em] uppercase">
            Iris Biometric Infrastructure — v1.0.0
          </span>
        </div>
        <div className="flex gap-8 text-[9px] font-mono font-bold tracking-[0.2em] uppercase text-zinc-600">
          <span className="hover:text-iris-purple cursor-pointer">
            Security
          </span>
          <span className="hover:text-iris-purple cursor-pointer">
            Protocol
          </span>
          <span className="hover:text-iris-purple cursor-pointer">Uptime</span>
          <span className="hover:text-iris-purple cursor-pointer">Audit</span>
        </div>
      </div>
    </footer>
  );
};

export default Footer;
