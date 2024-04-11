using Google.Protobuf;

namespace CheeseAPI.Controller
{
    public class RequestData(string code, IList<string> data)
    {
        public string code = code;
        public IList<string> data = data;
    }
}
