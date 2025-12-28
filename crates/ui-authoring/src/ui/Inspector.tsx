/**
 * Inspector panel for viewing and editing node properties.
 *
 * Shows parameters and boundary outputs for the selected node.
 */

import React from 'react';
import type { UINode, UIParamValue, UIBoundaryOutput } from '../graph/internalModel';

export interface InspectorProps {
  selectedNode: UINode | null;
  boundaryOutputs: UIBoundaryOutput[];
  onParamChange?: (nodeId: string, paramKey: string, value: UIParamValue) => void;
  onAddBoundaryOutput?: (nodeId: string, portName: string, outputName: string) => void;
  onRemoveBoundaryOutput?: (outputName: string) => void;
}

export const Inspector: React.FC<InspectorProps> = ({
  selectedNode,
  boundaryOutputs,
  onParamChange,
  onAddBoundaryOutput,
  onRemoveBoundaryOutput,
}) => {
  if (!selectedNode) {
    return (
      <div style={styles.container}>
        <div style={styles.header}>Inspector</div>
        <div style={styles.empty}>Select a node to inspect</div>
      </div>
    );
  }

  const nodeOutputs = boundaryOutputs.filter(o => o.nodeId === selectedNode.id);

  return (
    <div style={styles.container}>
      <div style={styles.header}>Inspector</div>

      {/* Node Info */}
      <div style={styles.section}>
        <div style={styles.sectionTitle}>Node</div>
        <div style={styles.field}>
          <span style={styles.label}>ID:</span>
          <span style={styles.value}>{selectedNode.id}</span>
        </div>
        <div style={styles.field}>
          <span style={styles.label}>Type:</span>
          <span style={styles.value}>{selectedNode.type}</span>
        </div>
        <div style={styles.field}>
          <span style={styles.label}>Version:</span>
          <span style={styles.value}>{selectedNode.version}</span>
        </div>
      </div>

      {/* Parameters */}
      <div style={styles.section}>
        <div style={styles.sectionTitle}>Parameters</div>
        {Object.entries(selectedNode.params).length === 0 ? (
          <div style={styles.empty}>No parameters</div>
        ) : (
          Object.entries(selectedNode.params).map(([key, param]) => (
            <div key={key} style={styles.paramRow}>
              <span style={styles.label}>{key}</span>
              <input
                style={styles.input}
                type={param.type === 'bool' ? 'checkbox' : 'text'}
                value={param.type === 'bool' ? undefined : String(param.value)}
                checked={param.type === 'bool' ? param.value : undefined}
                onChange={(e) => {
                  if (!onParamChange) return;
                  let newValue: UIParamValue;
                  switch (param.type) {
                    case 'bool':
                      newValue = { type: 'bool', value: e.target.checked };
                      break;
                    case 'number':
                      newValue = { type: 'number', value: parseFloat(e.target.value) || 0 };
                      break;
                    case 'int':
                      newValue = { type: 'int', value: parseInt(e.target.value) || 0 };
                      break;
                    default:
                      newValue = { type: param.type, value: e.target.value };
                  }
                  onParamChange(selectedNode.id, key, newValue);
                }}
              />
            </div>
          ))
        )}
      </div>

      {/* Boundary Outputs */}
      <div style={styles.section}>
        <div style={styles.sectionTitle}>Boundary Outputs</div>
        {nodeOutputs.length === 0 ? (
          <div style={styles.empty}>No outputs exposed</div>
        ) : (
          nodeOutputs.map(output => (
            <div key={output.name} style={styles.outputRow}>
              <span style={styles.outputName}>{output.name}</span>
              <span style={styles.outputPort}>:{output.portName}</span>
              {onRemoveBoundaryOutput && (
                <button
                  style={styles.removeButton}
                  onClick={() => onRemoveBoundaryOutput(output.name)}
                >
                  ×
                </button>
              )}
            </div>
          ))
        )}
      </div>
    </div>
  );
};

const styles: Record<string, React.CSSProperties> = {
  container: {
    width: 280,
    backgroundColor: '#1e1e2e',
    borderLeft: '1px solid #3d3d5c',
    display: 'flex',
    flexDirection: 'column',
    color: '#e2e8f0',
    fontSize: 13,
  },
  header: {
    padding: '12px 16px',
    borderBottom: '1px solid #3d3d5c',
    fontWeight: 600,
    fontSize: 14,
  },
  section: {
    padding: '12px 16px',
    borderBottom: '1px solid #3d3d5c',
  },
  sectionTitle: {
    fontSize: 11,
    fontWeight: 600,
    textTransform: 'uppercase',
    color: '#64748b',
    marginBottom: 8,
  },
  field: {
    marginBottom: 6,
  },
  label: {
    color: '#94a3b8',
    marginRight: 8,
  },
  value: {
    color: '#e2e8f0',
  },
  empty: {
    color: '#64748b',
    fontStyle: 'italic',
    padding: '8px 0',
  },
  paramRow: {
    display: 'flex',
    alignItems: 'center',
    justifyContent: 'space-between',
    marginBottom: 8,
  },
  input: {
    backgroundColor: '#2d2d44',
    border: '1px solid #4a4a6a',
    borderRadius: 4,
    color: '#e2e8f0',
    padding: '4px 8px',
    width: 100,
  },
  outputRow: {
    display: 'flex',
    alignItems: 'center',
    marginBottom: 6,
  },
  outputName: {
    color: '#6366f1',
    fontWeight: 500,
  },
  outputPort: {
    color: '#64748b',
    marginLeft: 4,
    flex: 1,
  },
  removeButton: {
    background: 'none',
    border: 'none',
    color: '#94a3b8',
    cursor: 'pointer',
    fontSize: 16,
    padding: '0 4px',
  },
};
