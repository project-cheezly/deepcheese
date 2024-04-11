namespace CheeseAPI.Controller.IndiBroker
{
    public class IndiHealthChecker : BaseIndiBroker
    {
        private readonly IConfiguration config;

        public IndiHealthChecker(ILogger<ControllerFactory> logger, IConfiguration config, SynchronizationContext ctx)
            : base(logger, ctx)
        {
            this.config = config;
            InitializeControl();
            StartIndi();
        }

        private void InitializeControl()
        {
            ctx.Send(_ =>
            {
                control = new AxGIEXPERTCONTROLLib.AxGiExpertControl();
                control.CreateControl();
            }, null);
        }

        public bool StartIndi()
        {
            bool result = false;
            ctx.Send(_ =>
            {
                result = control?.StartIndi(
                    config.GetValue<string>("Auth:ID"),
                    config.GetValue<string>("Auth:Password"),
                    config.GetValue<string>("Auth:CertPassword"),
                    config.GetValue<string>("Auth:BinaryPath")
                ) ?? false;
            }, null);

            return result;
        }

        public bool CloseIndi()
        {
            bool result = false;
            ctx.Send(_ =>
            {
                result = control?.CloseIndi() ?? false;
            }, null);
            return result;
        }
    }
}
