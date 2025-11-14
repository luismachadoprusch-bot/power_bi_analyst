-- Banco: ProcessAutomation (ex.: CREATE DATABASE ProcessAutomation;)
USE ProcessAutomation;
GO

-- Usuários (opcional, para responsabilizar tarefas)
CREATE TABLE dbo.[User] (
    UserId INT IDENTITY(1,1) PRIMARY KEY,
    UserName NVARCHAR(100) NOT NULL,
    Email NVARCHAR(256) NULL,
    IsActive BIT NOT NULL DEFAULT 1,
    CreatedAt DATETIME2 NOT NULL DEFAULT SYSUTCDATETIME()
);

-- Processos (alto nível)
CREATE TABLE dbo.ProcessInstance (
    ProcessId INT IDENTITY(1,1) PRIMARY KEY,
    ProcessName NVARCHAR(200) NOT NULL,
    InitiatorUserId INT NULL REFERENCES dbo.[User](UserId),
    Status NVARCHAR(50) NOT NULL DEFAULT 'Running', -- Running, Completed, Cancelled
    StartedAt DATETIME2 NOT NULL DEFAULT SYSUTCDATETIME(),
    CompletedAt DATETIME2 NULL,
    CorrelationId UNIQUEIDENTIFIER DEFAULT NEWID() -- útil para Power Platform
);

-- Tarefas/Atividades do processo
CREATE TABLE dbo.ProcessTask (
    TaskId INT IDENTITY(1,1) PRIMARY KEY,
    ProcessId INT NOT NULL REFERENCES dbo.ProcessInstance(ProcessId) ON DELETE CASCADE,
    TaskName NVARCHAR(200) NOT NULL,
    TaskType NVARCHAR(100) NULL, -- ex: Approval, HumanTask, ServiceTask
    AssignedToUserId INT NULL REFERENCES dbo.[User](UserId),
    Status NVARCHAR(50) NOT NULL DEFAULT 'Pending', -- Pending, InProgress, Completed, Skipped
    Payload NVARCHAR(MAX) NULL, -- JSON com dados do task
    CreatedAt DATETIME2 NOT NULL DEFAULT SYSUTCDATETIME(),
    StartedAt DATETIME2 NULL,
    CompletedAt DATETIME2 NULL,
    Priority INT NOT NULL DEFAULT 5
);

-- Histórico/auditoria de tarefas (append-only)
CREATE TABLE dbo.TaskHistory (
    HistoryId BIGINT IDENTITY(1,1) PRIMARY KEY,
    TaskId INT NOT NULL REFERENCES dbo.ProcessTask(TaskId),
    ChangeType NVARCHAR(50) NOT NULL, -- Created, Assigned, Started, Completed, StatusChanged
    PrevStatus NVARCHAR(50) NULL,
    NewStatus NVARCHAR(50) NULL,
    ChangedByUserId INT NULL REFERENCES dbo.[User](UserId),
    Comment NVARCHAR(1000) NULL,
    CreatedAt DATETIME2 NOT NULL DEFAULT SYSUTCDATETIME()
);

-- Fila de integração para Power Platform (Power Automate pode fazer Polling ou receber via trigger)
CREATE TABLE dbo.IntegrationQueue (
    QueueId BIGINT IDENTITY(1,1) PRIMARY KEY,
    EventType NVARCHAR(100) NOT NULL, -- e.g., TaskCompleted, TaskCreated, ProcessStarted
    Payload NVARCHAR(MAX) NULL, -- JSON com dados necessários
    IsProcessed BIT NOT NULL DEFAULT 0,
    CreatedAt DATETIME2 NOT NULL DEFAULT SYSUTCDATETIME(),
    ProcessedAt DATETIME2 NULL
);
