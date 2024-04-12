using CheeseAPI.Controller.IndiBroker;

namespace CheeseAPI.Controller
{
    public class ControllerFactory
    {
        private readonly ILogger<ControllerFactory> logger;
        private readonly IConfiguration config;

        private SynchronizationContext? ctx;
        private IndiHealthChecker indiHealthChecker;

        public ControllerFactory(ILogger<ControllerFactory> logger, IConfiguration config)
        {
            this.logger = logger;
            this.config = config;

            Thread thread = new(_ =>
            {
                var control = new AxshinhanINDI64Lib.AxshinhanINDI64();
                control.CreateControl();

                ctx = new WindowsFormsSynchronizationContext();
                Application.Run();
            });

            thread.SetApartmentState(ApartmentState.STA);
            thread.Start();

            while (ctx is null)
            {
                Thread.Sleep(1000);
                logger.LogInformation("SynchronizationContext 생성 대기중...");
            }

            logger.LogInformation("SynchronizationContext 생성 완료");
            indiHealthChecker = new IndiHealthChecker(logger, config, ctx!);
        }

        public LookupController CreateLookupController()
        {
            return new LookupController(logger, ctx!);
        }

        public RealtimeController CreateRealtimeController()
        {
            return new RealtimeController(logger, ctx!);
        }

        public IndiHealthChecker CreateIndiHealthChecker()
        {
            return indiHealthChecker;
        }
    }
}
