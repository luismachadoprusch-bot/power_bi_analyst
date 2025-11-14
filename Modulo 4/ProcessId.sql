-- 1) Criar processo e retornar ProcessId
CREATE PROCEDURE dbo.sp_CreateProcess
    @ProcessName NVARCHAR(200),
    @InitiatorUserId INT = NULL,
    @OutProcessId INT OUTPUT
AS
BEGIN
    SET NOCOUNT ON;
    INSERT INTO dbo.ProcessInstance (ProcessName, InitiatorUserId)
    VALUES (@ProcessName, @InitiatorUserId);

    SET @OutProcessId = SCOPE_IDENTITY();

    -- fila para integração
    INSERT INTO dbo.IntegrationQueue (EventType, Payload)
    VALUES ('ProcessStarted', CONCAT('{"ProcessId":', @OutProcessId, ',"ProcessName":"', @ProcessName, '"}'));
END
GO

-- 2) Adicionar tarefa
CREATE PROCEDURE dbo.sp_AddTask
    @ProcessId INT,
    @TaskName NVARCHAR(200),
    @TaskType NVARCHAR(100) = NULL,
    @AssignedToUserId INT = NULL,
    @Payload NVARCHAR(MAX) = NULL,
    @OutTaskId INT OUTPUT
AS
BEGIN
    SET NOCOUNT ON;
    INSERT INTO dbo.ProcessTask (ProcessId, TaskName, TaskType, AssignedToUserId, Payload)
    VALUES (@ProcessId, @TaskName, @TaskType, @AssignedToUserId, @Payload);

    SET @OutTaskId = SCOPE_IDENTITY();

    INSERT INTO dbo.TaskHistory (TaskId, ChangeType, NewStatus, ChangedByUserId, Comment)
    VALUES (@OutTaskId, 'Created', 'Pending', NULL, NULL);

    INSERT INTO dbo.IntegrationQueue (EventType, Payload)
    VALUES ('TaskCreated', @Payload);
END
GO

-- 3) Iniciar / setar tarefa como InProgress (claim)
CREATE PROCEDURE dbo.sp_StartTask
    @TaskId INT,
    @UserId INT
AS
BEGIN
    SET NOCOUNT ON;
    DECLARE @prevStatus NVARCHAR(50);
    SELECT @prevStatus = Status FROM dbo.ProcessTask WHERE TaskId = @TaskId;

    UPDATE dbo.ProcessTask
    SET Status = 'InProgress', AssignedToUserId = @UserId, StartedAt = SYSUTCDATETIME()
    WHERE TaskId = @TaskId;

    INSERT INTO dbo.TaskHistory (TaskId, ChangeType, PrevStatus, NewStatus, ChangedByUserId)
    VALUES (@TaskId, 'Started', @prevStatus, 'InProgress', @UserId);

    INSERT INTO dbo.IntegrationQueue (EventType, Payload)
    VALUES ('TaskStarted', CONCAT('{"TaskId":', @TaskId, ',"UserId":', @UserId, '}'));
END
GO

-- 4) Completar tarefa
CREATE PROCEDURE dbo.sp_CompleteTask
    @TaskId INT,
    @UserId INT,
    @ResultPayload NVARCHAR(MAX) = NULL -- JSON com resultado/decisão
AS
BEGIN
    SET NOCOUNT ON;
    DECLARE @prevStatus NVARCHAR(50), @procId INT;
    SELECT @prevStatus = Status, @procId = ProcessId FROM dbo.ProcessTask WHERE TaskId = @TaskId;

    UPDATE dbo.ProcessTask
    SET Status = 'Completed', CompletedAt = SYSUTCDATETIME(), Payload = COALESCE(Payload, @ResultPayload)
    WHERE TaskId = @TaskId;

    INSERT INTO dbo.TaskHistory (TaskId, ChangeType, PrevStatus, NewStatus, ChangedByUserId, Comment)
    VALUES (@TaskId, 'Completed', @prevStatus, 'Completed', @UserId, NULL);

    INSERT INTO dbo.IntegrationQueue (EventType, Payload)
    VALUES ('TaskCompleted', CONCAT('{"TaskId":', @TaskId, ',"ProcessId":', @procId, ',"UserId":', @UserId, ',"Result":', JSON_QUERY(COALESCE(@ResultPayload,'null')), '}'));
END
GO

-- 5) Mark process completed (concludente)
CREATE PROCEDURE dbo.sp_CompleteProcess
    @ProcessId INT
AS
BEGIN
    SET NOCOUNT ON;
    UPDATE dbo.ProcessInstance
    SET Status = 'Completed', CompletedAt = SYSUTCDATETIME()
    WHERE ProcessId = @ProcessId;

    INSERT INTO dbo.IntegrationQueue (EventType, Payload)
    VALUES ('ProcessCompleted', CONCAT('{"ProcessId":', @ProcessId, '}'));
END
GO
